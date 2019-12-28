## Дополнение Г - Средства разработки

В этом дополнении мы расскажем про часто используемые средства разработки, предоставляемые Rust. Мы рассмотрим автоматическое форматирование, быстрый путь исправления предупреждений, линтер, и интеграцию с IDE.

### Автоматическое форматирование с `rustfmt`

Инструмент `rustfmt` отформатирует Ваш код в соответствии с принятым в сообществе стилем.
Многие совместные проекты используют `rustfmt` для предотвращения споров об используемом стиле для написания кода на Rust: каждый форматирует свой код с помощью этой утилиты.

Для установки `rustfmt`, введите следующее:

```text
$ rustup component add rustfmt
```

Эта команда установит `rustfmt` и `cargo-fmt`, также как Rust дает Вам одновременно `rustc` и `cargo`. Для форматирования проекта, использующего Cargo, введите следующее:

```text
$ cargo fmt
```

Эта команда отформатирует весь код на языке Rust в текущем крейте. Будет изменен только стиль кода, семантика останется прежней. Для большей информации о `rustfmt`, смотрите [документацию](https://github.com/rust-lang/rustfmt).

### Исправление кода с `rustfix`

The rustfix tool is included with Rust installations and can automatically fix
some compiler warnings. If you’ve written code in Rust, you’ve probably seen
compiler warnings. For example, consider this code:

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

```text
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

Предупреждение предлагает нам использовать `_i` как имя переменной: нижнее подчеркивание в начале идентификатора предполагает, что мы его не используем. Мы можем автоматически применить это предположение с помощью `rustfix`, запустив команду `cargo fix`:

```text
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

Инструмент Clippy является набором проверок (lints) для анализа Вашего кода, поэтому Вы можете найти простые ошибки и улучшить ваш Rust код.

Для установки Clippy, введите следующее:

```text
$ rustup component add clippy
```

Для запуска проверок Clippy’s для проекта Cargo, введите следующее:

```text
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
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/master/index.html#approx_constant
```

Эта ошибка сообщает Вам, что эта константа уже определена более точно в Rust, и используя ее Ваша программа будет работать более правильно. Затем Вы бы изменили Ваш код, добавив константу `PI`. Следующий код не вызовет каких-либо ошибок или предупреждений от Clippy:

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Для большей информации о Clippy смотрите [документацию](https://github.com/rust-lang/rust-clippy).

### Интеграция с IDE используя Rust Language Server

Для помощи в интеграции с IDE, Rust распространяет *Rust Language Server* (`rls`). Этот инструмент говорит на [Language Server Protocol](http://langserver.org/), который является спецификацией для общения между IDE и языками программирования. Различные клиенты могут использовать `rls`, например [плагин Rust для Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).

Для установки `rls`, введите следующее:

```text
$ rustup component add rls
```

Затем установите поддержу языкового сервера в Вашей IDE. Вы получите такие вещи, как автодополнение, просмотр определения и подсветку ошибок.

Для большей информации про `rls`, смотрите [документацию](https://github.com/rust-lang/rls).
