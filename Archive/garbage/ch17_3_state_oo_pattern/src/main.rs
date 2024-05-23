use ch17_3_state_oo_pattern::Post; // Random crate for example

fn main() {
    let mut post = Post::new();

    // add_text() when its in draft mode
    post.add_text("Lorem ipsum dolor sit amet, consectetur adipiscing el");
    assert_eq!("", post.content()); // as Post haven't been published and it is in `draft` state.

    post.request_review();
    assert_eq!("", post.content()); // Review has been requested, the post is still not published

    post.approve();
    assert_eq!(
        "Lorem ipsum dolor sit amet, consectetur adipiscing el",
        post.content()
    );

    /*
    here <lib1> is BETTER than <lib>
     */
}
