pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub содержимое: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub reply: bool,
    pub resocialpost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.имя_пользователя, self.content)
    }
}

// ANCHOR: here
pub fn notify(предмет: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// ANCHOR_END: here
