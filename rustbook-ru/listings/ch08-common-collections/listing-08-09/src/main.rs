fn main() {
    // ANCHOR: here
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("синий")),
        SpreadsheetCell::Float(10.12),
    ];
    // ANCHOR_END: here
}
