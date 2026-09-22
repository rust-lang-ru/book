fn main() {
    // ANCHOR: here
    fn увеличить_на_единицу(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // ANCHOR_END: here

    let пять = Some(5);
    let шесть = увеличить_на_единицу(пять);
    let none = увеличить_на_единицу(None);
}
