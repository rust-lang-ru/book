mod back_of_house {
    pub struct Breakfast {
        pub хлеб: String,
        сезонный_фрукт: String,
    }

    impl Breakfast {
        pub fn summer(хлеб: &str) -> Breakfast {
            Breakfast {
                хлеб: String::from(хлеб),
                сезонный_фрукт: String::from("персики"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye хлеб
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.хлеб = String::from("Пшеница");
    println!("Я хочу {} хлеб ,пожалуйста", meal.хлеб);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.сезонный_фрукт = String::from("голубойberries");
}
