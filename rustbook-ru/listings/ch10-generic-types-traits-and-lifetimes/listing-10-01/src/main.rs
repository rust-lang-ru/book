// ANCHOR: here
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut наибольшее = &number_list[0];

    for number in &number_list {
        if number > наибольшее {
            наибольшее = number;
        }
    }

    println!("The наибольшее number is {наибольшее}");
    // ANCHOR_END: here
    assert_eq!(*наибольшее, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
