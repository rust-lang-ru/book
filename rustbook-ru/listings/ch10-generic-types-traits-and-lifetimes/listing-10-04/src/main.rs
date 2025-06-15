// ANCHOR: here
fn наибольший_i32(список: &[i32]) -> &i32 {
    let mut наибольшее = &список[0];

    for предмет in список {
        if предмет > наибольшее {
            наибольшее = предмет;
        }
    }

    наибольшее
}

fn наибольший_char(список: &[char]) -> &char {
    let mut наибольшее = &список[0];

    for предмет in список {
        if предмет > наибольшее {
            наибольшее = предмет;
        }
    }

    наибольшее
}

fn main() {
    let список_чисел = vec![34, 50, 25, 100, 65];

    let итог = наибольший_i32(&список_чисел);
    println!("Наибольшее число {итог}");
    // ANCHOR_END: here
    assert_eq!(*result, 100);
    // ANCHOR: here

    let char_list = vec!['y', 'm', 'a', 'q'];

    let итог = наибольший_char(&char_list);
    println!("Наибольший знак {итог}");
    // ANCHOR_END: here
    assert_eq!(*result, 'y');
    // ANCHOR: here
}
// ANCHOR_END: here
