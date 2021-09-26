//正式的内容
pub struct Post {
  content: String,
}

//草稿内容
pub struct DraftPost {
  content: String,
}

//等待审批中的内容
pub struct PendingReviewPost {
  content: String,
}

//正式发布后的Post
impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

//草稿Post
impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

//等待审批的Post中只有通过审批的方法，审批通过之后直接将正式的Post内容返回
impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}
