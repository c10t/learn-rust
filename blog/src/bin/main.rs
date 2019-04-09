extern crate blog;
use blog::Post;
use blog::Post2;

fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());

  // Encoding States and Behavior as Types
   
  let mut post2 = Post2::new();

  post2.add_text("I ate a salad for lunch today");
  let post2 = post2.request_review();
  let post2 = post2.approve();
  assert_eq!("I ate a salad for lunch today", post2.content());
}
