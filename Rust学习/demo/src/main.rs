struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn getX(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point { x: 1, y: 2 };
  // x is: 1
  println!("x is: {}", p.getX());
}
