extern crate trpl; // требуется для mdbook test

// ANCHOR: all
use trpl::{Either, Html};

fn main() {
    let свойства: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = заголовок_страницы(&свойства[1]);
        let title_fut_2 = заголовок_страницы(&свойства[2]);

        let (ссылка, есть_ли_заголовок) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{ссылка} returned first");
        match есть_ли_заголовок {
            Some(заголовок) => println!("Эта страница имеет заголовок: '{заголовок}'"),
            None => println!("Этот заголовок не может быть получен."),
        }
    })
}

async fn заголовок_страницы(ссылка: &str) -> (&str, Option<String>) {
    let содержимое = trpl::get(ссылка).await.text().await;
    let title = Html::parse(&содержимое)
        .select_first("заголовок")
        .map(|title| title.inner_html());
    (ссылка, заголовок)
}
// ANCHOR_END: all
