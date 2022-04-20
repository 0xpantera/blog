use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("test text");
    
    let post = post.request_review();

    let post = post.approve();

    assert_eq!("test text", post.content());
}