use std::clone::Clone;
use std::fmt::{Debug, Display};

pub trait Description {
  fn description(&self) -> String;
  fn default_fn(&self) -> String {
    format!(
      "description and default function test: {}",
      self.description()
    )
  }
}

pub struct People {
  pub name: String,
  pub gender: String,
  pub age: i32,
  pub height: i32,
}

impl Description for People {
  fn description(&self) -> String {
    //People: 没有进行默认实现的方法: 大锤, 18, 男, 200
    format!(
      "没有进行默认实现的方法: {}, {}, {}, {}",
      self.name, self.age, self.gender, self.height
    )
  }
}

pub fn test_trait_fn1<T: Description + Display, U: Debug + Clone>(item1: T, item2: U) {
  println!("{}", item1.description())
}

pub fn test_trait_fn2<T, U>(item1: T, item2: U)
where
  T: Description + Display,
  U: Debug + Clone,
{
  println!("{}", item1.description())
}

pub fn test() -> impl Description {
  People {
    name: String::from("王大锤"),
    gender: String::from("男"),
    age: 18,
    height: 200,
  }
}
