fn main() {
  let x = vec![1, 2, 3, 4];

  let test_closure = move |y| x == y;

  let y = vec![1, 2, 3, 4];
  println!("闭包调用的值：{}", test_closure(y));

  //报错：value moved into closure here，值的所有权已经被闭包夺去了
  println!("x:{:?}", x);
}
