mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let зазказ1 = back_of_house::Appetizer::Soup;
    let зазказ2 = back_of_house::Appetizer::Salad;
}
