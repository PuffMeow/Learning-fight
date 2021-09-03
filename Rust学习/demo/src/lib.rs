mod top_module {
  pub mod inner_module {
    pub fn test_fn() {
      println!("hello, world");
    }
    fn private_fn() {}
  }
}

use crate::top_module::inner_module;

pub fn test() {
  inner_module::test_fn();
  //不能引用私有的函数
  inner_module::private_fn();
}
