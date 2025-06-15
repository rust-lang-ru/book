// ANCHOR: here
fn наибольшее(список: &[i32]) -> &i32 {
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

    let итог = наибольшее(&список_чисел);
    println!("Наибольшее число {итог}");
    // ANCHOR_END: here
    assert_eq!(*result, 100);
    // ANCHOR: here

    let список_чисел = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let итог = наибольшее(&список_чисел);
    println!("Наибольшее число {итог}");
    // ANCHOR_END: here
    assert_eq!(*result, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
