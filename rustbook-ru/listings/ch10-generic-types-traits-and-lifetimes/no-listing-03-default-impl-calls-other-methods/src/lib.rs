// ANCHOR: here
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// ANCHOR_END: here

pub struct Tweet {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub reply: bool,
    pub retweet: bool,
}

// ANCHOR: impl
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.имя_пользователя)
    }
}
// ANCHOR_END: impl
