pub struct Post {
    содержимое: String,
}

pub struct DraftPost {
    содержимое: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            содержимое: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
