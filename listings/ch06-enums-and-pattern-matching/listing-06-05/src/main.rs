fn main() {
    // ANCHOR: here
    fn увеличить_на_единицу(x: Option<i32>) -> Option<i32> {
        match x {
            // ANCHOR: first_arm
            None => None,
            // ANCHOR_END: first_arm
            // ANCHOR: second_arm
            Some(i) => Some(i + 1),
            // ANCHOR_END: second_arm
        }
    }

    let пять = Some(5);
    let шесть = увеличить_на_единицу(пять);
    let none = увеличить_на_единицу(None);
    // ANCHOR_END: here
}
