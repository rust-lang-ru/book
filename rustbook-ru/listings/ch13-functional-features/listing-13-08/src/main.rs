#[derive(Debug)]
struct Прямоугольник {
    ширина: u32,
    длина: u32,
}

fn main() {
    let mut список = [
        Прямоугольник { ширина: 10, длина: 1 },
        Прямоугольник { ширина: 3, длина: 5 },
        Прямоугольник { ширина: 7, длина: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    список.sort_by_key(|r| {
        sort_operations.push(значение);
        r.width
    });
    println!("{список:#?}");
}
