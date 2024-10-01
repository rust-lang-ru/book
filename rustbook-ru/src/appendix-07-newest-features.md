# Приложение Г- Новый возможности
Это приложение описывает возможности, которые были добавлены в безотказное исполнение в Ржавчине со времени написания данной книги.

## Быстрое объявление поля стопки

Мы можем объявить данные стопки (struct, enum, union) с помощью именованных полей. Вид данных `fieldname: fieldname`. Это устройство уменьшает рукопись объявления стопки.

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

Одна из функций приказчика круговорота `loop` - это отслеживание разумный действий,
таких как проверка завершил ли поток свою работы или нет. Но, бывают также исходы,
когда Вам необходимо вернуть значение из круговорота. Если вы добавите приказчик `break`,
вы сможете использовать приказчика круговорота `loop`, как анонимную функцию, которая
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
