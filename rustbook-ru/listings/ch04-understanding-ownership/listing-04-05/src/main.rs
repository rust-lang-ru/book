fn main() {
    let s1 = String::from("здравствуй");

    let (s2, длина) = подсчёт_длины(s1);

    println!("Длина '{s2}' {длина}.");
}

fn подсчёт_длины(s: String) -> (String, usize) {
    let длина = s.len(); // len() returns длина a String

    (s, длина)
}
