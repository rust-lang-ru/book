pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub содержимое: String,
}

impl Summary for NewsArticle {}

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
