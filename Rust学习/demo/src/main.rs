use std::fmt::Display;

fn longest_mark<'a, T>(x: &'a str, y: &'a str, mark: T) -> &'a str
where
  T: Display,
{
  println!("mark is {}", mark);
  if (x.len() > y.len()) {
    x
  } else {
    y
  }
}

fn main() {}
