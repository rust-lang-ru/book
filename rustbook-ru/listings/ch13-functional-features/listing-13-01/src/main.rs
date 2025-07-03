#[derive(Debug, PartialEq, Copy, Clone)]
enum ЦветРубашки {
    Красный,
    Синий,
}

struct Inventory {
    рубашки: Vec<ЦветРубашки>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ЦветРубашки>) -> ЦветРубашки {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ЦветРубашки {
        let mut num_red = 0;
        let mut num_голубой = 0;

        for цвет in &self.рубашки {
            match цвет {
                ЦветРубашки::Красный => num_red += 1,
                ЦветРубашки::Синий => num_голубой += 1,
            }
        }
        if num_red > num_голубой {
            ЦветРубашки::Красный
        } else {
            ЦветРубашки::Синий
        }
    }
}

fn main() {
    let store = Inventory {
        рубашки: vec![ЦветРубашки::Синий, ЦветРубашки::Красный, ЦветРубашки::Синий],
    };

    let user_pref1 = Some(ЦветРубашки::Красный);
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
