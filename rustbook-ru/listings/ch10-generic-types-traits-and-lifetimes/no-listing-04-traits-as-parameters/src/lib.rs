pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub заголовок: String,
    pub местонахождение: String,
    pub author: String,
    pub содержимое: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.заголовок, self.author, self.местонахождение)
    }
}

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

// ANCHOR: here
pub fn notify(предмет: &impl Summary) {
    println!("Взрывные новости! {}", item.summarize());
}
// ANCHOR_END: here
