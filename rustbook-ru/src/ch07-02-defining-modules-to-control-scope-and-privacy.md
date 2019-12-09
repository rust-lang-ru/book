## Определение модулей для контроля видимости и конфиденциальности

In this section, we’ll talk about modules and other parts of the module system,
namely *paths* that allow you to name items; the `use` keyword that brings a
path into scope; and the `pub` keyword to make items public. We’ll also discuss
the `as` keyword, external packages, and the glob operator. For now, let’s
focus on modules!

*Modules* let us organize code within a crate into groups for readability and
easy reuse. Modules also control the *privacy* of items, which is whether an
item can be used by outside code (*public*) or is an internal implementation
detail and not available for outside use (*private*).

В качестве примера, давайте напишем библиотечный крейт предоставляющий функциональность ресторана. Мы определим сигнатуры функций, но оставим их тела пустыми, чтобы сосредоточиться на организации кода, вместо реализации кода для ресторана.

In the restaurant industry, some parts of a restaurant are referred to as
*front of house* and others as *back of house*. Front of house is where
customers are; this is where hosts seat customers, servers take orders and
payment, and bartenders make drinks. Back of house is where the chefs and cooks
work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can
organize the functions into nested modules. Create a new library named
`restaurant` by running `cargo new --lib restaurant`; then put the code in
Listing 7-1 into *src/lib.rs* to define some modules and function signatures.

<span class="filename">Файл: src/lib.rs</span>

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

<span class="caption">Листинг 7-1. Модуль <code>front_of_house</code> содержащий другие модули, которые затем содержат функции</span>

We define a module by starting with the `mod` keyword and then specify the
name of the module (in this case, `front_of_house`) and place curly brackets
around the body of the module. Inside modules, we can have other modules, as in
this case with the modules `hosting` and `serving`. Modules can also hold
definitions for other items, such as structs, enums, constants, traits, or—as
in Listing 7-1—functions.

Используя модули, можно сгруппировать связанные определения вместе и названия. Программистам, использующим такой код, будет легче найти определения, которые они хотели использовать, потому что они могли бы ориентироваться в сгруппированном коде, вместо того, чтобы прочитать все определения. Программисты, добавляющие новые функции в этот код, будут знать, где разместить код для поддержания порядка в программе.

Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate
roots. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the *module tree*.

Listing 7-2 shows the module tree for the structure in Listing 7-1.

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">Листинг 7-2. Дерево модулей для кода в листинге 7-1</span>

This tree shows how some of the modules nest inside one another (for example,
`hosting` nests inside `front_of_house`). The tree also shows that some modules
are *siblings* to each other, meaning they’re defined in the same module
(`hosting` and `serving` are defined within `front_of_house`). To continue the
family metaphor, if module A is contained inside module B, we say that module A
is the *child* of module B and that module B is the *parent* of module A.
Notice that the entire module tree is rooted under the implicit module named
`crate`.

Дерево модулей может напомнить вам дерево каталогов файловой системы на компьютере; это очень удачное сравнение! Так же, как каталоги в файловой системе, для организации кода используются модули. И так же, как для файлов в каталоге, нужен способ найти модули.
