mod тыл_дома {
    pub struct Поесть {
        pub хлеб: String,
        сезонный_фрукт: String,
    }

    impl Поесть {
        pub fn лето(хлеб: &str) -> Поесть {
            Поесть {
                хлеб: String::from(хлеб),
                сезонный_фрукт: String::from("персики"),
            }
        }
    }
}

pub fn поесть_в_ресторане() {
    // Order a поесть in the лето with Rye хлеб
    let mut meal = тыл_дома::Поесть::лето("Rye");
    // Change our mind about what bread we'd like
    meal.хлеб = String::from("Пшеница");
    println!("Я хочу {} хлеб ,пожалуйста", meal.хлеб);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.сезонный_фрукт = String::from("голубойberries");
}
