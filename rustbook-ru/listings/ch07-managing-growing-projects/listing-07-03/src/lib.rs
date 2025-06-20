mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Безусловный путь
    crate::front_of_house::hosting::add_to_waitlist();

    // Относительный путь
    front_of_house::hosting::add_to_waitlist();
}
