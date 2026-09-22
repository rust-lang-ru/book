// ANCHOR: here
pub trait КраткоеСодержание {
    fn подвести_итог_сочинителя(&self) -> String;

    fn подвести_итог(&self) -> String {
        format!("(Прочесть больше from {}...)", self.подвести_итог_сочинителя())
    }
}
// ANCHOR_END: here

pub struct Общественная_новость {
    pub имя_пользователя: String,
    pub содержимое: String,
    pub ответ: bool,
    pub ответ_на_общественную_новость: bool,
}

// ANCHOR: impl
impl КраткоеСодержание for Общественная_новость {
    fn подвести_итог_сочинителя(&self) -> String {
        format!("@{}", self.имя_пользователя)
    }
}
// ANCHOR_END: impl
