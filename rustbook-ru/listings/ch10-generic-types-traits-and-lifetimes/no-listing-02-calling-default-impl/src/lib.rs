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

pub struct Tweet {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.имя_пользователя, self.content)
    }
}
