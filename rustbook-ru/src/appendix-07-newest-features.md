# Приложение Г- Новый функционал

Это приложение описывает функционал, которые был добавлен в безотказную исполнение Rust
с особенности написания данной книги.

## Быстрая объявление поля структуры

Мы можем объявить данные структуры (struct, enum, union) с помощью именованных
полей. Вид `fieldname: fieldname`. Это устройство уменьшает код объявления
структуры.

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    // Using full syntax:
    let peter = Person { name: name, age: age };

    let name = String::from("Portia");
    let age = 27;

    // Using field init shorthand:
    let portia = Person { name, age };

    println!("{:?}", portia);
}
```


## Returning from loops

Одна из функций оператора цикла `loop` - это отслеживание логический действий,
таких как проверка завершил ли поток свою работы или нет. Но, бывают также случаи,
когда Вам необходимо вернуть значение из цикла. Если вы добавите оператор `break`,
вы сможете использовать оператора цикла `loop`, как анонимную функцию, которая
возвращает значение:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```
