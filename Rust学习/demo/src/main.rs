use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: PartialOrd + Display> Pair<T> {
  fn cmp_display(&self) {
    if (self.x > self.y) {
      println!("x > y")
    } else {
      println!("x < y")
    }
  }
}

fn main() {}
