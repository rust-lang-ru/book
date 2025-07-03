struct ВажнаяВыдержка<'a> {
    часть: &'a str,
}

// ANCHOR: 1st
impl<'a> ВажнаяВыдержка<'a> {
    fn уровень(&self) -> i32 {
        3
    }
}
// ANCHOR_END: 1st

// ANCHOR: 3rd
impl<'a> ВажнаяВыдержка<'a> {
    fn announce_and_return_part(&self, объявление: &str) -> &str {
        println!("Внимание ,пожалуйста: {объявление}");
        self.часть
    }
}
// ANCHOR_END: 3rd

fn main() {
    let роман = String::from("Звал меня Измаил. Несколько лет назад...");
    let первое_предложение = роман.split('.').next().unwrap();
    let i = ВажнаяВыдержка {
        часть: первое_предложение,
    };
}
