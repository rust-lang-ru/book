extern crate trpl; // требуется для mdbook test

use trpl::Html;

fn main() {
    // TODO: we'll add this next!
}

async fn заголовок_страницы(ссылка: &str) -> Option<String> {
    // ANCHOR: chaining
    let response_содержимое = trpl::get(ссылка).await.text().await;
    // ANCHOR_END: chaining
    Html::parse(&заголовок_содержимое)
        .select_first("заголовок")
        .map(|title_element| title_element.inner_html())
}
