## Дополнение Г - Средства разработки

В этом дополнении мы расскажем про часто используемые средства разработки, предоставляемые Rust. Мы рассмотрим автоматическое форматирование, быстрый путь исправления предупреждений, линтер, и интеграцию с IDE.

### Автоматическое форматирование с `rustfmt`

Инструмент `rustfmt` переформатирует ваш код в соответствии со стилем кода сообщества. Многие совместные проекты используют `rustfmt`, чтобы предотвратить споры о том, какой стиль использовать при написании Rust: все форматируют свой код с помощью этого инструмента.

Для установки `rustfmt`, введите следующее:

```console
$ rustup component add rustfmt
```

Эта команда установит `rustfmt` и `cargo-fmt`, также как Rust даёт Вам одновременно `rustc` и `cargo`. Для форматирования проекта, использующего Cargo, введите следующее:

```console
$ cargo fmt
```

Эта команда отформатирует весь код на языке Rust в текущем крейте. Будет изменён только стиль кода, семантика останется прежней. Для большей информации о `rustfmt`, смотрите [документацию].

### Исправление кода с `rustfix`

Инструмент rustfix включён в установку Rust и может автоматически исправлять предупреждения компилятора с очевидным способом исправления проблемы, скорее всего, подходящим вам. Вероятно, вы уже видели предупреждения компилятора. Например, рассмотрим этот код:

<span class="filename">Файл: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

Мы вызываем функцию `do_something` 100 раз, но никогда не используем переменную `i` в теле цикла `for`. Rust предупреждает нас об этом:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

Предупреждение предлагает нам использовать `_i` как имя переменной: нижнее подчёркивание в начале идентификатора предполагает, что мы его не используем. Мы можем автоматически применить это предположение с помощью `rustfix`, запустив команду `cargo fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Когда посмотрим в *src/main.rs* снова, мы увидим что `cargo fix` изменил наш код:

<span class="filename">Файл: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

Переменная цикла `for` теперь носит имя `_i`, и предупреждение больше не появляется.

Также Вы можете использовать команду `cargo fix` для перемещения вашего кода между различными редакциями Rust. Редакции будут рассмотрены в дополнении Д.

### Больше проверок с Clippy

Инструмент Clippy является коллекцией проверок (lints) для анализа Вашего кода, поэтому Вы можете найти простые ошибки и улучшить ваш Rust код.

Для установки Clippy, введите следующее:

```console
$ rustup component add clippy
```

Для запуска проверок Clippy’s для проекта Cargo, введите следующее:

```console
$ cargo clippy
```

Например, скажем что Вы хотите написать программу, в которой будет использоваться приближенная математическая константа, такая как число Пи, как в следующей программе:

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Запуск `cargo clippy` для этого проекта вызовет следующую ошибку:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Эта ошибка сообщает вам, что в Rust уже определена более точная константа `PI`, и что ваша программа будет более корректной, если вы вместо неё будете использовать эту константу. Затем вы должны изменить свой код, чтобы использовать константу `PI`. Следующий код не приводит к ошибкам или предупреждениям от Clippy:

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Для большей информации о Clippy смотрите [документацию](https://github.com/rust-lang/rustfmt).

### Интеграция с IDE с помощью `rust-analyzer`

Чтобы облегчить интеграцию с IDE, сообщество Rust рекомендует использовать [`rust-analyzer`]<!-- ignore -->. Этот инструмент представляет собой набор ориентированных на компилятор утилит, которые используют [Language Server Protocol]<!-- ignore -->, который является спецификацией для взаимодействия IDE и языков программирования друг с другом. Разные клиенты могут использовать `rust-analyzer`, например [подключаемый модуль анализатора Rust для Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

Посетите <a>домашнюю страницу</a> проекта <code>rust-analyzer</code> для получения инструкций по установке, затем установите поддержку языкового сервера в конкретной среде IDE. Ваша IDE получит такие возможности, как автозаполнение, переход к определению и встроенные ошибки.


[документацию]: https://github.com/rust-lang/rustfmt
[Language Server Protocol]: http://langserver.org/
[`rust-analyzer`]: https://rust-analyzer.github.io