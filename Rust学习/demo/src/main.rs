// 报错： doesn't have a size known at compile-time
// 编译时不知道它的大小
// fn returns_closure() -> Fn(i32) -> i32 {
//   |x| x + 1
// }

//解决方式
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn main() {}
