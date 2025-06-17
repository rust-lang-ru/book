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
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            содержимое: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            имя_пользователя: String::from("horse_ebooks"),
            содержимое: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            resocialpost: false,
        }
    }
}
// ANCHOR_END: here
