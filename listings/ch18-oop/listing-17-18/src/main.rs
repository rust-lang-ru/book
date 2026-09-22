use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a салат на обед сегодня");
    assert_eq!("", post.содержимое());

    post.request_review();
    assert_eq!("", post.содержимое());

    post.approve();
    assert_eq!("I ate a салат на обед сегодня", post.содержимое());
}
