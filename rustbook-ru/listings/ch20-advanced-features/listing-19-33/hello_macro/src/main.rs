use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn здравствуй_macro() {
        println!("Здравствуй, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
