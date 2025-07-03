fn main() {
    // ANCHOR: here
    let s1 = String::from("Здравствуй, ");
    let s2 = String::from("мир!");
    let s3 = s1 + &s2; // примечание: строка s1 было передано во владение s3 и более не доступно
                       // ANCHOR_END: here
}
