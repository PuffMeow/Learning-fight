use demo::Config; //引入Config块
use std::env; //环境变量相关的模块
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("解析参数时发生了错误");
    process::exit(1);
  });

  //demo是项目名称
  if let Err(e) = demo::run(config) {
    println!("程序发生错误：{}", e);
    process::exit(1);
  };
}
