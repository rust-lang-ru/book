use aggregator::{Summary, SocialPost};

fn main() {
    let socialpost = SocialPost {
        имя_пользователя: String::from("horse_ebooks"),
        содержимое: String::from(
            "конечно, как вы, вероятно, уже знаете, люди",
        ),
        ответ: false,
        resocialpost: false,
    };

    println!("1 new socialpost: {}", socialpost.summarize());
}
