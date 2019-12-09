## Пакеты и крейты

Первые части модульной системы, которые мы рассмотрим - это пакеты и крейты. Крейт - это двоичный файл или библиотека. *Корень крейта* - это исходный файл, из которого Rust компилятор запускается и составляет корневой модуль вашего крейта (мы объясним модули в разделе [«Определение модулей для управления областью видимости и конфиденциальности"](ch07-02-defining-modules-to-control-scope-and-privacy.html)<comment>). <em data-md-type="emphasis">Пакет</em> состоит из одного или нескольких крейтов, которые предоставляют набор функций. Пакет содержит файл <em data-md-type="emphasis">Cargo.toml</em> описывающий, как собрать эти крейты.</comment>

То что может содержать пакет определяют несколько правил. Пакет *должен* содержать ни одной или одну крейт библиотеку, и не более. Он может содержать столько исполняемых крейтов, сколько  хотите, но он должен содержать по крайней мере один исполняемый крейт).

Let’s walk through what happens when we create a package. First, we enter the
command `cargo new`:

```text
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

When we entered the command, Cargo created a *Cargo.toml* file, giving us a
package. Looking at the contents of *Cargo.toml*, there’s no mention of
*src/main.rs* because Cargo follows a convention that *src/main.rs* is the
crate root of a binary crate with the same name as the package. Likewise, Cargo
knows that if the package directory contains *src/lib.rs*, the package contains
a library crate with the same name as the package, and *src/lib.rs* is its
crate root. Cargo passes the crate root files to `rustc` to build the library
or binary.

Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a library and a binary, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.

Крейт группирует  в области видимости связанные вместе функциональности, поэтому функциональность легко распределить между несколькими проектами. Например, `rand` крейт, который мы использовали в [Главе 2](ch02-00-guessing-game-tutorial.html#generating-a-random-number)<comment> обеспечивает функциональность генерации случайных чисел. Можно использовать эту функциональность в наших собственных проектах, привнося крейт `rand` в область видимости проекта. Вся функциональность предоставляемая крейтом `rand` доступна через имя крейта `rand`</comment>

Keeping a crate’s functionality in its own scope clarifies whether particular
functionality is defined in our crate or the `rand` crate and prevents
potential conflicts. For example, the `rand` crate provides a trait named
`Rng`. We can also define a `struct` named `Rng` in our own crate. Because a
crate’s functionality is namespaced in its own scope, when we add `rand` as a
dependency, the compiler isn’t confused about what the name `Rng` refers to. In
our crate, it refers to the `struct Rng` that we defined. We would access the
`Rng` trait from the `rand` crate as `rand::Rng`.

Let’s move on and talk about the module system!
