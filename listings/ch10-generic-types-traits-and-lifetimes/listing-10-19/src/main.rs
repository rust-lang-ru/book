fn main() {
    let строка1 = String::from("abcd");
    let строка2 = "xyz";

    let итог = наибольшее(строка1.as_str(), строка2);
    println!("Наибольшая строка {итог}");
}
