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

    let mut num_sort_operations = 0;
    список.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{список:#?}, упорядочено {num_sort_operations} действиями");
}
