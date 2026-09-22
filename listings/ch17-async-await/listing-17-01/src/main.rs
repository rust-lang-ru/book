extern crate trpl; // требуется для mdbook test

fn main() {
    // TODO: we'll add this next!
}

// ANCHOR: all
use trpl::Html;

async fn заголовок_страницы(ссылка: &str) -> Option<String> {
    let response = trpl::get(ссылка).await;
    let response_содержимое = response.text().await;
    Html::parse(&заголовок_содержимое)
        .select_first("заголовок")
        .map(|title_element| title_element.inner_html())
}
// ANCHOR_END: all
