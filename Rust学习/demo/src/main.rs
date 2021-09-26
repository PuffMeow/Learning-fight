use demo::Post;

fn main() {
  //创建博客
  let mut post = Post::new();

  //博客发布内容
  post.add_text("我今天在学Rust");

  //博客请求审批
  let post = post.request_review();

  //通过博客审批
  let post = post.approve();

  //我今天在学Rust
  println!("{}", post.content());
}
