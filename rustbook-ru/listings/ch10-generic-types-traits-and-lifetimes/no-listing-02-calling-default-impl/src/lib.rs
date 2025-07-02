pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Прочесть больше...)")
    }
}

pub struct NewsArticle {
    pub заголовок: String,
    pub местонахождение: String,
    pub author: String,
    pub содержимое: String,
}

impl Summary for NewsArticle {}

pub struct SocialPost {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub ответ: bool,
    pub resocialpost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.имя_пользователя, self.содержимое)
    }
}
