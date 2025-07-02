pub struct Post {
    state: Option<Box<dyn State>>,
    содержимое: String,
}

// ANCHOR: here
impl Post {
    // --snip--
    // ANCHOR_END: here
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            содержимое: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.содержимое.push_str(text);
    }

    // ANCHOR: here
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().содержимое(self)
    }
    // --snip--
    // ANCHOR_END: here

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    // ANCHOR: here
}
// ANCHOR_END: here

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
