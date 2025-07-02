use aggregator::{self, NewsArticle, Summary};

fn main() {
    // ANCHOR: here
    let article = NewsArticle {
        заголовок: String::from("Penguins win the Stanley Cup Championship!"),
        местонахождение: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        содержимое: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockkey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    // ANCHOR_END: here
}
