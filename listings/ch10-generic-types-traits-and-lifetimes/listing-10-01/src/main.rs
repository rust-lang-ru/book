// ANCHOR: here
fn main() {
    let список_чисел = vec![34, 50, 25, 100, 65];

    let mut наибольшее = &список_чисел[0];

    for число in &список_чисел {
        if число > наибольшее {
            наибольшее = число;
        }
    }

    println!("Наибольшее число {наибольшее}");
    // ANCHOR_END: here
    assert_eq!(*наибольшее, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
