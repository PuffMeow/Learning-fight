use std::error::Error;
use std::fs; //文件系统相关模块

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_name: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    Ok(Config { query, file_name })
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "小小鸟";
    //注意前面不要留空格
    let content = "\
我是一只小小小小鸟
想要飞却怎么也飞不高
~~~~~~";

    //这里可以测试通过
    assert_eq!(vec!["我是一只小小小小鸟"], search(query, content));
  }
}
