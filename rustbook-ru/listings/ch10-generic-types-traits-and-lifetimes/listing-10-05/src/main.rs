fn наибольшее<T>(список: &[T]) -> &T {
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

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = наибольшее(&char_list);
    println!("The наибольшее char is {result}");
}
