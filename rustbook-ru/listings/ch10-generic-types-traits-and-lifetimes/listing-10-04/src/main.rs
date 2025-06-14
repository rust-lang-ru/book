// ANCHOR: here
fn largest_i32(список: &[i32]) -> &i32 {
    let mut наибольшее = &список[0];

    for item in список {
        if item > наибольшее {
            наибольшее = item;
        }
    }

    наибольшее
}

fn largest_char(список: &[char]) -> &char {
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

    let result = largest_i32(&number_list);
    println!("The наибольшее number is {result}");
    // ANCHOR_END: here
    assert_eq!(*result, 100);
    // ANCHOR: here

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The наибольшее char is {result}");
    // ANCHOR_END: here
    assert_eq!(*result, 'y');
    // ANCHOR: here
}
// ANCHOR_END: here
