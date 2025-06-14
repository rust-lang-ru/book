use std::ops::Deref;

impl<T> Deref for МойКороб<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct МойКороб<T>(T);

impl<T> МойКороб<T> {
    fn new(x: T) -> МойКороб<T> {
        МойКороб(x)
    }
}

fn здравствуй(имя: &str) {
    println!("Здравствуй, {имя}!");
}

// ANCHOR: here
fn main() {
    let m = МойКороб::new(String::from("Ржавчина"));
    здравствуй(&m);
}
// ANCHOR_END: here
