#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Голубой,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_голубой = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Голубой => num_голубой += 1,
            }
        }
        if num_red > num_голубой {
            ShirtColor::Red
        } else {
            ShirtColor::Голубой
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Голубой, ShirtColor::Red, ShirtColor::Голубой],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "Пользователь выбрал {:?} получил {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "Пользователь выбрал {:?} получил {:?}",
        user_pref2, giveaway2
    );
}
