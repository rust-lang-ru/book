extern crate trpl; // требуется для mdbook test

use trpl::Html;

// ANCHOR: run
fn main() {
    let свойства: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let ссылка = &свойства[1];
        match заголовок_страницы(ссылка).await {
            Some(заголовок) => println!("Заголовок для {ссылка} was {заголовок}"),
            None => println!("{ссылка} не имеет заголовка"),
        }
    })
}
// ANCHOR_END: run

async fn заголовок_страницы(ссылка: &str) -> Option<String> {
    let response_содержимое = trpl::get(ссылка).await.text().await;
    Html::parse(&заголовок_содержимое)
        .select_first("заголовок")
        .map(|title_element| title_element.inner_html())
}
