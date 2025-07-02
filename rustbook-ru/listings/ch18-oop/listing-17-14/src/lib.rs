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
        ""
    }
}
// ANCHOR_END: here

trait State {}

struct Draft {}

impl State for Draft {}
