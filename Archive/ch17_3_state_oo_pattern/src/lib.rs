pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match self.state.as_ref() {
            Some(s) => return s.content(self),
            None => return "",
        };
        // self.content.as_str()
    }

    pub fn request_review(&mut self) {
        // Need to modify the POST instance.
        if let Some(s) = self.state.take()
        /*Removes the current `state` - places NONE in its place*/
        /*This also gives us the ownership of `state`*/
        {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 1. We take ownership of the object.
        // 2. We discard the current state `self`.
        // 3. annotate a new state.
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Since NO review request has been made, So can't do anything.
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Don't need to do anything as its already in review state.
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // We need to transition it to a new state...
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // We can;t do any review as the Post is already published.
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Post is already published; No need to approve anything.
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
