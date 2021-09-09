use std::env;
use std::error::Error;
use std::fs; //文件系统相关模块

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;
  println!("{}", !config.case_sensitive);
  let results = if !config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_name: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    //使用CASE_INSENSITIVE环境变量，意思就是不区分大小写
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      file_name,
      case_sensitive,
    })
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

//为了不区分大小写，我们把搜索和要搜索的内容全部转换成小写就可以了
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  //区分大小写
  #[test]
  fn case_sensitive() {
    let query = "fly";
    //注意前面不要留空格
    let content = "\
I am a little bird.
Thought I want to fly.
I can't fly hight.
Fly into the sky.
~~~~~~";

    //这里可以测试通过
    assert_eq!(
      vec!["Thought I want to fly.", "I can't fly hight."],
      search(query, content)
    );
  }

  //不区分大小写
  #[test]
  fn case_insensitive() {
    let query = "FLY";
    //注意前面不要留空格
    let content = "\
I am a little bird.
Thought I want to fly.
I can't fly hight.
Fly into the sky.
~~~~~~";

    //这里可以测试通过
    assert_eq!(
      vec![
        "Thought I want to fly.",
        "I can't fly hight.",
        "Fly into the sky."
      ],
      search_case_insensitive(query, content)
    );
  }
}
