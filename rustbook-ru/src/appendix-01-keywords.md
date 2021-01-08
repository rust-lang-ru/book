## Дополнение А: Ключевые слова

Следующий список содержит ключевые слова, зарезервированные для текущего или будущего использования в Rust. Таким образом, данные слова нельзя использовать для идентификаторов (кроме сырых идентификаторов, которые мы обсудили в разделе "[Сырые идентификаторы](#raw-identifiers)<!--  -->"), включая имена функций, переменных, параметров, полей структур, модулей, пакетов, констант, макросов, статических переменных, атрибутов, типов, типажей или времён жизни.

### Ключевые слова, использующиеся в Rust настоящее время

Следующие ключевые слова в настоящее время имеют описанную функциональность.

- `as` - perform primitive casting, disambiguate the specific trait containing an item, or rename items in `use` and `extern crate` statements
- `async` -  return a `Future` instead of blocking the current thread
- `await` - suspend execution until the result of a `Future` is ready
- `break` - exit a loop immediately
- `const` - define constant items or constant raw pointers
- `continue` - continue to the next loop iteration
- `crate` - link an external crate or a macro variable representing the crate in which the macro is defined
- `dyn` - dynamic dispatch to a trait object
- `else` - fallback for `if` and `if let` control flow constructs
- `enum` - define an enumeration
- `extern` - link an external crate, function, or variable
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
- `union` - define a [union](../reference/items/unions.html) and is only a keyword when used in a union declaration
- `unsafe` - denote unsafe code, functions, traits, or implementations
- `use` - bring symbols into scope
- `where` - denote clauses that constrain a type
- `while` - loop conditionally based on the result of an expression

### Keywords Reserved for Future Use

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

### Raw Identifiers

*Сырые идентификаторы* - это синтаксис, позволяющий вам использовать ключевые слова там, где обычно они не могут быть. Для создания и использования сырого идентификатора к ключевому слову добавляется префикс `r#`.

For example, `match` is a keyword. If you try to compile the following function that uses `match` as its name:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

you’ll get this error:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

The error shows that you can’t use the keyword `match` as the function identifier. To use `match` as a function name, you need to use the raw identifier syntax, like this:

<span class="filename">Filename: src/main.rs</span>

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
