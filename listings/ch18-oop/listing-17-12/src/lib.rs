pub struct Post {
    state: Option<Box<dyn State>>,
    содержимое: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            содержимое: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
