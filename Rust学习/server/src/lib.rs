use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

enum Message {
  NewJob(Job),
  //终止执行
  Terminate,
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

type Job = Box<FnBox + Send + 'static>;

pub struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl Worker {
  fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = reciver.lock().unwrap().recv().unwrap();
      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; excuting.", id);

          job.call_box();
        }
        Message::Terminate => {
          println!("Worker terminate, {}", id);
          break;
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }
    println!("Shutting down all worker.");
    for worker in &mut self.workers {
      println!("Shutting down worker: {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let (sender, reciver) = mpsc::channel();
    let reciver = Arc::new(Mutex::new(reciver));
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&reciver)));
    }
    ThreadPool { workers, sender }
  }

  pub fn excute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}
