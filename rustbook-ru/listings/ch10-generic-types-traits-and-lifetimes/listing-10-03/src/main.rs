// ANCHOR: here
fn наибольшее(список: &[i32]) -> &i32 {
    let mut наибольшее = &список[0];

    for item in список {
        if item > наибольшее {
            наибольшее = item;
        }
    }

    наибольшее
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = наибольшее(&number_list);
    println!("The наибольшее number is {result}");
    // ANCHOR_END: here
    assert_eq!(*result, 100);
    // ANCHOR: here

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = наибольшее(&number_list);
    println!("The наибольшее number is {result}");
    // ANCHOR_END: here
    assert_eq!(*result, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
