## Дополнение А: Ключевые слова

Следующий список содержит ключевые слова, зарезервированные для текущего или будущего использования в Rust. Таким образом, данные слова нельзя использовать для идентификаторов (кроме сырых идентификаторов, которые мы обсудили в разделе "[Сырые идентификаторы](#raw-identifiers)<!--  -->"), включая имена функций, переменных, параметров, полей структур, модулей, пакетов, констант, макросов, статических переменных, атрибутов, типов, типажей или времён жизни.

### Ключевые слова, использующиеся в Rust настоящее время

Следующие ключевые слова в настоящее время имеют описанную функциональность.

- `as` - perform primitive casting, disambiguate the specific trait containing an item, or rename items in `use` and `extern crate` statements
- `async` -  return a `Future` instead of blocking the current thread
- `await` - suspend execution until the result of a `Future` is ready
- `break` - немедленное прекращение цикла
- `const` - объявляет константу или константный сырой указатель
- `continue` - перейти к следующей итерации цикла
- `crate` - link an external crate or a macro variable representing the crate in which the macro is defined
- `dyn` - динамическая диспетчеризация для трейт-объектов
- `else` - альтернатива для `if` и `if let`
- `enum` - определение перечисления
- `extern` - определение использования внешнего пакета, функции или переменной
- `false` - логический литерал false
- `fn` - определение функции или типа-указателя на функцию
- `for` - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
- `if` - условный оператор ветвления
- `impl` - наследование или реализация трейта
- `in` - часть синтаксической конструкции цикла `for`
- `let` - определение, привязывание переменной
- `loop` - бесконечный цикл
- `match` - оператор сопоставления значения с образцом
- `mod` - оператор определения модуля
- `move` - позволяет замыканию брать во владение всё, что "захватывает" замыкание
- `mut` - обозначение изменяемых переменных, ссылок, сырых указателей или привязок к шаблону
- `pub` - обозначение публичного доступа к полям структуры, `impl` блокам или модулям
- `ref` - ссылочное связывание
- `return` - оператор возврата из функции
- `Self` - a type alias for the type we are defining or implementing
- `self` - предмет метода или текущий модуль
- `static` - глобальная переменная или время жизни, продолжающееся всё время работы программы
- `struct` - определение структуры
- `super` - родительский модуль относительно текущего
- `trait` - обозначение трейта
- `true` - логический литерал true
- `type` - объявление псевдонима типа или ассоциированного типа
- `union` - define a [union](../reference/items/unions.html) and is only a keyword when used in a union declaration
- `unsafe` - определение небезопасного кода, функции, трейта или реализаций
- `use` - оператор импорта символов в текущую область видимости
- `where` - оператор условия-ограничения для типа
- `while` - условный цикл, основанный на результате вычисления выражения

### Ключевые слова, зарезервированные для будущего использования

The following keywords do not have any functionality but are reserved by Rust for potential future use.

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### Сырые идентификаторы

*Сырые идентификаторы* - это синтаксис, позволяющий вам использовать ключевые слова там, где обычно они не могут быть. Для создания и использования сырого идентификатора к ключевому слову добавляется префикс `r#`.

For example, `match` is a keyword. If you try to compile the following function that uses `match` as its name:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

вы получите ошибку:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

The error shows that you can’t use the keyword `match` as the function identifier. To use `match` as a function name, you need to use the raw identifier syntax, like this:

<span class="filename">Файл: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

This code will compile without any errors. Note the `r#` prefix on the function name in its definition as well as where the function is called in `main`.

Сырые идентификаторы позволяют использовать любые слова <br>в качестве идентификатора, даже если это зарезервированное <br>слово. Дополнительно сырые идентификаторы позволяют использовать библиотеки, написанные на отличной от используемой вами редакции Rust. Например `try`  является ключевым словом в 2018 редакции, но не в 2015. Если вы зависите от библиотеки, написанной с использованием 2015 редакции и имеющей функцию `try`, то для вызова такой функции из кода 2018 редакции, вам необходимо использовать синтаксис сырых идентификаторов. В данном случае `r#try`. Больше подробностей про редакции можно найти в [Приложении Е](appendix-05-editions.html)<!-- . -->
