struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let оба_целых_числа = Point { x: 5, y: 10 };
    let оба_плавающие_числа = Point { x: 1.0, y: 4.0 };
    let целое_число_и_вещественное = Point { x: 5, y: 4.0 };
}
