## Вставка имён

Мы изучили, как вызывать функции, определённые в звене, используя имена звеньев, как
часть вызова. Пример 7-6:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

<span class="caption">Пример 7-6: Вызов функции, указав полный к ней путь</span>

Как вы видите, указание полного пути к функции весьма утомительно. Конечно же, в Rust
имеется возможности упрощающий вызов функций.

### Краткий вставка. Использование `use`

Использование ключевого слова `use` сокращает указание полного пути к функции, которую
вы хотите использовать в определённой области видимости. Пример применения `use`:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

Строка `use a::series::of;` указывает, что в данной области видимости могут использоваться элементы,
которые находятся в звене `of`. Их можно вызывать просто указывая приставка имени этого
звена `of::`.

В область видимости попадают только элементы звена. Подчиненные звене не включаются.
Если в этом будет необходимость - надо явным образом это указать.
Поэтому укажем `of::nested_modules`, вместо `nested_modules`.

Чтобы не указывать имя звена, можно выполнить постоянной вставка функции в
область видимости:

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}
use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

Такой способ, даёт нам возможность, сокращать список подключения.

Очень важная возможность подключения значений перечислений!
Т.к. перечисления можно назвать разновидностью пространств имён, то можно указать
только необходимые элементы перечисления при вставкае:

```rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("{:?}", red);
    println!("{:?}", yellow);
    println!("{:?}", green);
}
```
Так как мы не включили `TrafficLight` в список вставленных значений перечисления,
то для его использования нам необходимо указать полный путь до этого элемента.

### Вставка всех элементов с помощью `*`

Есть ли возможность подключения всех элементов выбранного пространства имён?!
Да. Есть. Используйте `*`:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```
Знак `*` называют *glob* и его функция - вставка всех элементов, видимых извне
пространства имён. Обратите также внимание, что наряду с удобствами, существуют
также недостатки использования полного подключения пространства имён, т.к. это может привести
к конфликтным или неожиданным случайм, когда в разных пространствах имён существуют
одинаковые (по имени) функции, которые будут вставляться.
Пример:

```rust
pub mod a {
    pub mod series {
        pub mod of1 {
            pub fn nested_modules() {
                println!("nested_modules 1");
            }
        }
        pub mod of2 {
            pub fn nested_modules() {
                println!("nested_modules 2");
            }
        }
    }
}
use a::series::of1::*;
use a::series::of2::*;

fn main() {
    nested_modules();
}
```

Описание ошибки:
```
error: `nested_modules` is ambiguous
  --> src/main.rs:19:5
   |
19 |     nested_modules();
   |     ^^^^^^^^^^^^^^
   |
note: `nested_modules` could refer to the name imported here
  --> src/main.rs:15:5
   |
15 | use a::series::of1::*;
   |     ^^^^^^^^^^^^^^^^^^
note: `nested_modules` could also refer to the name imported here
  --> src/main.rs:16:5
   |
16 | use a::series::of2::*;
   |     ^^^^^^^^^^^^^^^^^^
   = note: consider adding an explicit import of `nested_modules` to disambiguate
```

### Доступ к возможности. родительского звена с помощью `super`

Как вы помните, при создании библиотеки, Cargo предлагает использовать звено `tests`.
Сейчас разберёмся подробнее. Добавим рукопись проверки в исходную рукопись файла *src/lib.rs*:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

В главе 11  подробно рассказывается о проверке. Сейчас мы только немного расскажем.
Обратите внимание на особую изложение и то, что это отдельный звено в нашей рукописи.
Состоящая из звеньев система нашего дела теперь имеет вид:

```text
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

Проверки помогают отлаживать рукопись библиотеки. Напишем наш первый проверку. Он будет вызывать
функцию `client::connect`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Выполнение проверок осуществляется приказом`cargo test`:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `client`
```

Почему-то сборка прошла неуспешно. Почему же? Нам не надо добавлять приставка
библиотеки `communicator::`, т.к. мы находимся внутри неё.

Как же вызвать функцию `client::connect` из звена `tests`? В звене `tests` мы
можем указать что мы хотим начать поиски звеньев с корневого звена:

```rust,ignore
::client::connect();
```

Или мы можем использовать `super` для того чтобы переместиться по состоящей из звеньев упорядочевания
на один уровень выше текущего звена:

```rust,ignore
super::client::connect();
```

Эти две возможности выглядят одинаковыми в этом примере. Если находитесь глубоко
внутри состоящей из звеньев упорядочевания, то начиная с корневого звена вашу рукопись будет длинным.
Есть случаи, когда использование `super` более удобно.

Это бывает утомительно печать `super::` в каждом проверке. Есть решение `use`.
Возможность `super::` изменяет путь, который вы используете в `use`.

Для тех случаев, когда вы пишите проверки к библиотекам, использование `use super::something`,
наилучшее решение.

Пример:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Когда вы теперь выполните приказ `cargo test`, вы увидите следующий вывод:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Итоги

Теперь вы знаете ещё один способ, как можно согласовать вашу рукопись. Его можно использовать
для объединения различных элементов вместе, при переработке большой рукописи.

Далее, мы рассмотрим устройства данных встроенной библиотеки.
