use aggregator::{Summary, SocialPost};

fn main() {
    let socialpost = SocialPost {
        имя_пользователя: String::from("horse_ebooks"),
        содержимое: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        resocialpost: false,
    };

    println!("1 new socialpost: {}", socialpost.summarize());
}
