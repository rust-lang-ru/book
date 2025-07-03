struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let не_работает = Point { x: 5, y: 4.0 };
}
