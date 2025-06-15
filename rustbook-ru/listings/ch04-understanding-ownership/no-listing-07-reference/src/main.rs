// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("здравствуй");

    let длина = подсчёт_длины(&s1);
    // ANCHOR_END: here

    println!("Длина '{s1}' {длина}.");
}

fn подсчёт_длины(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
