#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut список = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    список.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{список:#?}, упорядочено {num_sort_operations} действиями");
}
