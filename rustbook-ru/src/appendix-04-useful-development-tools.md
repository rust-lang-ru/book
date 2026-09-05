## Приложение D: полезные инструменты разработки

В этом приложении мы поговорим о некоторых полезных инструментах разработки,
которые предоставляет проект Rust. Мы рассмотрим автоматическое форматирование,
быстрые способы применять исправления предупреждений, линтер и интеграцию с
IDE.

### Автоматическое форматирование с помощью `rustfmt`

Инструмент `rustfmt` переформатирует ваш код в соответствии со стилем кода
сообщества. Многие совместные проекты используют `rustfmt`, чтобы избежать
споров о том, какой стиль использовать при написании Rust: все форматируют свой
код с помощью этого инструмента.

Установки Rust по умолчанию включают `rustfmt`, поэтому программы `rustfmt` и
`cargo-fmt` уже должны быть в вашей системе. Эти две команды аналогичны `rustc`
и `cargo` в том смысле, что `rustfmt` позволяет более тонко управлять
форматированием, а `cargo-fmt` понимает соглашения проекта, использующего
Cargo. Чтобы отформатировать любой проект Cargo, введите следующее:

```console
$ cargo fmt
```

Выполнение этой команды переформатирует весь код Rust в текущем крейте. Это
должно изменить только стиль кода, а не семантику кода. Дополнительные сведения
о `rustfmt` см. в [его документации][rustfmt].

### Исправление кода с помощью `rustfix`

Инструмент `rustfix` входит в установки Rust и может автоматически исправлять
предупреждения компилятора, для которых есть ясный способ исправить проблему,
и это, скорее всего, именно то, что вам нужно. Вероятно, вы уже видели
предупреждения компилятора. Например, рассмотрим этот код:

<span class="filename">Имя файла: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Здесь мы определяем переменную `x` как изменяемую, но на самом деле никогда ее
не изменяем. Rust предупреждает нас об этом:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

Предупреждение предлагает удалить ключевое слово `mut`. Мы можем автоматически
применить это предложение с помощью инструмента `rustfix`, выполнив команду
`cargo fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Когда мы снова посмотрим на _src/main.rs_, увидим, что `cargo fix` изменил код:

<span class="filename">Имя файла: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

Переменная `x` теперь неизменяемая, и предупреждение больше не появляется.

Команду `cargo fix` также можно использовать для перехода вашего кода между
разными редакциями Rust. Редакции рассматриваются в [приложении E][editions]<!--
ignore -->.

### Больше линтов с Clippy

Инструмент Clippy — это набор линтов для анализа вашего кода, чтобы вы могли
находить распространенные ошибки и улучшать свой код Rust. Clippy входит в
стандартные установки Rust.

Чтобы запустить линты Clippy для любого проекта Cargo, введите следующее:

```console
$ cargo clippy
```

Например, предположим, что вы пишете программу, использующую приближение
математической константы, такой как pi, как делает эта программа:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Выполнение `cargo clippy` в этом проекте приводит к такой ошибке:

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

Эта ошибка сообщает, что в Rust уже определена более точная константа `PI`, и
что ваша программа была бы корректнее, если бы вы использовали эту константу.
Затем вы изменили бы код, чтобы использовать константу `PI`.

Следующий код не вызывает у Clippy никаких ошибок или предупреждений:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Дополнительные сведения о Clippy см. в [его документации][clippy].

### Интеграция с IDE с помощью `rust-analyzer`

Чтобы помочь с интеграцией IDE, сообщество Rust рекомендует использовать
[`rust-analyzer`][rust-analyzer]<!-- ignore -->. Этот инструмент представляет
собой набор ориентированных на компилятор утилит, которые говорят на [Language
Server Protocol][lsp]<!-- ignore --> — спецификации, позволяющей IDE и языкам
программирования взаимодействовать друг с другом. `rust-analyzer` могут
использовать разные клиенты, например [плагин Rust analyzer для Visual Studio
Code][vscode].

Посетите [домашнюю страницу][rust-analyzer]<!-- ignore --> проекта
`rust-analyzer`, чтобы получить инструкции по установке, затем установите
поддержку языкового сервера в вашей конкретной IDE. Ваша IDE получит такие
возможности, как автодополнение, переход к определению и встроенные ошибки.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: appendix-05-editions.md
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
