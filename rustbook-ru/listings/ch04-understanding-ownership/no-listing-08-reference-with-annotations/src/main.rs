fn main() {
    let s1 = String::from("здравствуй");

    let длина = подсчёт_длины(&s1);

    println!("Длина '{s1}' {длина}.");
}

// ANCHOR: here
fn подсчёт_длины(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
// ANCHOR_END: here
