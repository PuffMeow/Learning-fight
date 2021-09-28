enum Message {
  Hello { id: i32 },
}

fn main() {
  let msg = Message::Hello { id: 5 };

  match msg {
    Message::Hello {
      id: id_variable @ 3..=7,
    } => println!("3到7的范围内找到id: {}", id_variable),
    Message::Hello { id: 10..=12 } => println!("id在10到12范围内"),
    Message::Hello { id } => println!("其它值"),
  }
}
