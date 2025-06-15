use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        имя_пользователя: String::from("horse_ebooks"),
        содержимое: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
