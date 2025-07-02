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
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            заголовок: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            местонахождение: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            содержимое: String::from(
                "The Pittsburgh Penguins once again are the best \
                 хоккейная команда в NHL.",
            ),
        }
    } else {
        SocialPost {
            имя_пользователя: String::from("horse_ebooks"),
            содержимое: String::from(
                "конечно, как вы, вероятно, уже знаете, люди",
            ),
            ответ: false,
            resocialpost: false,
        }
    }
}
// ANCHOR_END: here
