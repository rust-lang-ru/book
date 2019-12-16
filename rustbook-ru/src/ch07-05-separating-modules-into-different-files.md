## Разделение модулей на разные файлы

Пока что все примеры в этой главе определяли множество модулей в одном файле. Когда модули становятся большими, можно переместить их определения в отдельный файл, чтобы сделать код проще.

For example, let’s start from the code in Listing 7-17 and move the
`front_of_house` module to its own file *src/front_of_house.rs* by changing the
crate root file so it contains the code shown in Listing 7-21. In this case,
the crate root file is *src/lib.rs*, but this procedure also works with binary
crates whose crate root file is *src/main.rs*.

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

<span class="caption">Листинг 7-21. Объявление модуля <code>front_of_house</code> тело которого будет в <em>src/front_of_house.rs</em></span>

And *src/front_of_house.rs* gets the definitions from the body of the
`front_of_house` module, as shown in Listing 7-22.

<span class="filename">Файл: src/front_of_house.rs</span>

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

<span class="caption">Листинг 7-22. Определения внутри модуля <code>front_of_house</code> в файле <em>src/front_of_house.rs</em></span>

Использование точки с запятой после `mod front_of_house` вместо объявления начала блока говорит Rust загрузить содержимое модуля из другого файла с указанным именем, что и название модуля. Чтобы продолжить с нашим примером и выделить модуль `hosting` в свой отдельный файл, мы меняем *src/front_of_house.rs* так, чтобы он содержал только объявление модуля `hosting`:

<span class="filename">Файл: src/front_of_house.rs</span>

```
pub mod hosting;
```

Then we create a *src/front_of_house* directory and a file
*src/front_of_house/hosting.rs* to contain the definitions made in the
`hosting` module:

<span class="filename">Файл: src/front_of_house/hosting.rs</span>

```
pub fn add_to_waitlist() {}
```

The module tree remains the same, and the function calls in `eat_at_restaurant`
will work without any modification, even though the definitions live in
different files. This technique lets you move modules to new files as they grow
in size.

Обратите внимание, что в выражение 
`pub use crate::front_of_house::hosting` в файле *src/lib.rs* также не изменилось и использование `use` не влияет на то, какие файлы копилируются как часть крейта. Ключевое слово `mod` объявляет модули, а Rust заглядывает в файл с тем же именем, что и модуль для кода, который входит в этот модуль.

## Итог

Rust lets you split a package into multiple crates and a crate into modules
so you can refer to items defined in one module from another module. You can do
this by specifying absolute or relative paths. These paths can be brought into
scope with a `use` statement so you can use a shorter path for multiple uses of
the item in that scope. Module code is private by default, but you can make
definitions public by adding the `pub` keyword.

В следующей главе мы рассмотрим некоторые структуры данных коллекций из стандартной библиотеки, которые можно использовать в своем аккуратно организованном коде.
