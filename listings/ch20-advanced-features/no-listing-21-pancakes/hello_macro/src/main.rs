use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn здравствуй_macro() {
        println!("Здравствуй, Макро! Меня зовут Блины!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
