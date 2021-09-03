mod top_module;

pub use crate::top_module::inner_module;

pub fn test_top_module() {
  inner_module::inner_module::test_fn();
}
