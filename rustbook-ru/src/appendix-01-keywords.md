## Приложение A: Ключевые слова

Следующий список содержит ключевые слова, зарезервированные для текущего или будущего использования в языке Rust. Как таковые их нельзя использовать в качестве идентификаторов (за исключением сырых идентификаторов, которые мы обсудим в разделе [«Сырые идентификаторы»](#raw-identifiers)<!-- ignore -->). Идентификаторы — это имена функций, переменных, параметров, полей структур, модулей, крейтов, констант, макросов, статических значений, атрибутов, типов, свойств или времён жизни.

### Используемые в настоящее время ключевые слова

Ниже приведён список используемых в настоящее время ключевых слов с описанием их функций.

-
- `as` — выполнить примитивное преобразование, уточнить конкретную характеристику, которую содержит объект, или переименовать элемент в выражении <code>use</code>
- `async` —  вернуть `Future` вместо блокирования текущего потока
- `await` — немедленно завершить цикл приостановить выполнение до тех пор, пока не будет готов результат `Future`
- `break` — немедленно завершить цикл
- `const` — определить константу или неизменяемый указатель
- `continue` — досрочно перейти к следующей итерации цикла
- `crate` — в пути модуля ссылается на корень пакета
- `dyn` — динамическая отсылка к объекту характеристики
- `else` — ветвь для конструкций потока управления `if` и `if let` в случае, если никакая другая ветвь не была исполнена
- `enum` — определить перечисление
- `extern` — подключить внешнюю функцию или переменную
- `false` — логический литерал «ложь»
- `fn` — определить функцию или тип указателя на функцию
- `for` — перебор элементов итератора, реализация типажа или указание срока жизни более продолжительного периода
- `if` — ветвление на основе результата условного выражения
- `impl` — реализовать функциональность непосредственно или через типаж
- `in` — часть синтаксиса определения цикла  `for`
- `let` — объявить переменную
- `loop` — безусловный (бесконечный) цикл
- `match` — сопоставить значение с образцами
- `mod` — определить модуль
- `move` — заставить замыкание принять на себя владение всеми своими захватами
- `mut` — обозначить изменяемость для ссылок, сырых указателей или привязок к шаблонам
- `pub` — обозначить публичную доступность полей структур, блоков `impl` или модулей
- `ref` — привязка по ссылке
- `return` — возврат из функции
- `Self` — псевдоним для типа, который мы определяем или реализуем
- `self` — объект, содержащий этот метод, или текущий модуль
- `static` — глобальная переменная или время жизни на протяжении всего выполнения программы
- `struct` — определить структуру
- `super` — родительский модуль текущего модуля
- `trait` — определить типаж
- `true` — логический литерал «истина»
- `type` — определить псевдоним типа или ассоциированный тип
- `union` — определение [union](../reference/items/unions.html)<!-- ignore --> (объединения); является ключевым словом только при использовании в объявлении объединения
- `unsafe` — обозначить небезопасный код, функции, типажа или реализации
- `use` — ввести объекты в область видимости
- `where` — обозначить утверждения, которые ограничивают тип
- `while` — цикл, работающий относительно результата условного выражения
- `async` -  return a `Future` instead of blocking the current thread
- `await` - suspend execution until the result of a `Future` is ready
- `break` - exit a loop immediately
- `const` - define constant items or constant raw pointers
- `continue` - continue to the next loop iteration
- `crate` - in a module path, refers to the crate root
- `dyn` - dynamic dispatch to a trait object
- `else` - fallback for `if` and `if let` control flow constructs
- `enum` - define an enumeration
- `extern` - link an external function or variable
- `false` - Boolean false literal
- `fn` - define a function or the function pointer type
- `for` - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
- `if` - branch based on the result of a conditional expression
- `impl` - implement inherent or trait functionality
- `in` - part of `for` loop syntax
- `let` - bind a variable
- `loop` - loop unconditionally
- `match` - match a value to patterns
- `mod` - define a module
- `move` - make a closure take ownership of all its captures
- `mut` - denote mutability in references, raw pointers, or pattern bindings
- `pub` - denote public visibility in struct fields, `impl` blocks, or modules
- `ref` - bind by reference
- `return` - return from function
- `Self` - a type alias for the type we are defining or implementing
- `self` - method subject or current module
- `static` - global variable or lifetime lasting the entire program execution
- `struct` - define a structure
- `super` - parent module of the current module
- `trait` - define a trait
- `true` - Boolean true literal
- `type` - define a type alias or associated type
- `union` - define a [union](../reference/items/unions.html)<!-- ignore -->; is only a keyword when used in a union declaration
- `unsafe` - denote unsafe code, functions, traits, or implementations
- `use` - bring symbols into scope
- `where` - denote clauses that constrain a type
- `while` - loop conditionally based on the result of an expression

### Ключевые слова, зарезервированные для будущего использования

Следующие ключевые слова ещё не имеют никакой функциональности, но зарезервированы Rust для возможного использования в будущем.

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

### Сырые идентификаторы<a id="raw-identifiers"></a>

*Сырые идентификаторы* — это синтаксис, позволяющий использовать ключевые слова там, где обычно они не могут быть. Для создания и использования сырого идентификатора к ключевому слову добавляется префикс `r#`.

Например, ключевое слово `match`. Если вы попытаетесь скомпилировать следующую функцию, использующую в качестве имени `match`:

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

Ошибка говорит о том, что вы не можете использовать ключевое слово `match` в качестве идентификатора функции. Чтобы получить возможность использования слова `match` в качестве имени функции, нужно использовать синтаксис «сырых идентификаторов», например так:

<span class="filename">Файл: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Этот код скомпилируется без ошибок. Обратите внимание, что префикс `r#` в определении имени функции указан так же, как он указан в месте её вызова в `main`.

Сырые идентификаторы позволяют вам использовать любое слово, которое вы выберете, в качестве идентификатора, даже если это слово окажется зарезервированным ключевым словом. Это даёт нам больше свободы в выборе имён идентификаторов, а также позволяет нам интегрироваться с программами, написанными на языке, где эти слова не являются ключевыми. Кроме того, необработанные идентификаторы позволяют вам использовать библиотеки, написанные в версии Rust, отличной от используемой в вашем крейте. Например, `try` не является ключевым словом в выпуске 2015 года, но есть в выпуске 2018 года. Если вы зависите от библиотеки, написанной с использованием версии 2015 года и имеющей функцию `try`, вам потребуется использовать синтаксис сырого идентификатора, в данном случае `r#try`, для вызова этой функции из кода версии 2018 года. См. [Приложение E](appendix-05-editions.html)<!-- ignore --> для получения дополнительной информации о редакциях Rust.
