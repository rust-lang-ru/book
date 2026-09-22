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
pub fn уведомление(предмет: &impl КраткоеСодержание) {
    println!("Взрывные новости! {}", предмет.подвести_итог());
}
// ANCHOR_END: here
