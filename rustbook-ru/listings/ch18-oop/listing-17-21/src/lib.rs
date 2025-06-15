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

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            содержимое: self.content,
        }
    }
}

pub struct PendingReviewPost {
    содержимое: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            содержимое: self.content,
        }
    }
}
