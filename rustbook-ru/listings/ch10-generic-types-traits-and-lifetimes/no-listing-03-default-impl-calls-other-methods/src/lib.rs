// ANCHOR: here
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Прочесть больше from {}...)", self.summarize_author())
    }
}
// ANCHOR_END: here

pub struct SocialPost {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub ответ: bool,
    pub resocialpost: bool,
}

// ANCHOR: impl
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.имя_пользователя)
    }
}
// ANCHOR_END: impl
