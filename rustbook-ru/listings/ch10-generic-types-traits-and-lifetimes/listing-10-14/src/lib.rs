// ANCHOR: here
pub trait КраткоеСодержание {
    fn подвести_итог(&self) -> String {
        String::from("(Прочесть больше...)")
    }
}
// ANCHOR_END: here

pub struct НовостнаяСтатья {
    pub заголовок: String,
    pub местонахождение: String,
    pub сочинитель: String,
    pub содержимое: String,
}

impl КраткоеСодержание for НовостнаяСтатья {}

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
