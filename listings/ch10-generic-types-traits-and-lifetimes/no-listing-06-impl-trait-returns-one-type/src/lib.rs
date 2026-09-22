pub trait КраткоеСодержание {
    fn подвести_итог(&self) -> String;
}

pub struct НовостнаяСтатья {
    pub заголовок: String,
    pub местонахождение: String,
    pub сочинитель: String,
    pub содержимое: String,
}

impl КраткоеСодержание for НовостнаяСтатья {
    fn подвести_итог(&self) -> String {
        format!("{}, by {} ({})", self.заголовок, self.сочинитель, self.местонахождение)
    }
}

pub struct Общественная_новость {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub ответ: bool,
    pub ответ_на_общественную_новость: bool,
}

impl КраткоеСодержание for Общественная_новость {
    fn подвести_итог(&self) -> String {
        format!("{}: {}", self.имя_пользователя, self.содержимое)
    }
}

// ANCHOR: here
fn returns_summarizable(switch: bool) -> impl КраткоеСодержание {
    if switch {
        НовостнаяСтатья {
            заголовок: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            местонахождение: String::from("Pittsburgh, PA, USA"),
            сочинитель: String::from("Iceburgh"),
            содержимое: String::from(
                "The Pittsburgh Penguins once again are the best \
                 хоккейная команда в NHL.",
            ),
        }
    } else {
        Общественная_новость {
            имя_пользователя: String::from("электронные_книги_о_лошадях"),
            содержимое: String::from(
                "конечно, как вы, вероятно, уже знаете, люди",
            ),
            ответ: false,
            ответ_на_общественную_новость: false,
        }
    }
}
// ANCHOR_END: here
