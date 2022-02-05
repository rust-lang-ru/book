## Определение и инициализация структур

Структуры похожи на кортежи, рассмотренные в разделе ["Тип Кортеж"](ch03-02-data-types.html#the-tuple-type), так как оба хранят несколько связанных значений. Как и кортежи, части структур могут быть разных типов. В отличие от кортежей, в структуре необходимо именовать каждую часть данных для понимания смысла значений. Добавление этих имён обеспечивает большую гибкость структур по сравнению с кортежами: не нужно полагаться на порядок данных для указания значений экземпляра или доступа к ним.

Для определения структуры указывается ключевое слово `struct` и её название. Название должно описывать значение частей данных, сгруппированных вместе. Далее, в фигурных скобках для каждой новой части данных поочерёдно определяются имя части данных и её тип. Каждая пара <code>имя: тип</code> называется *полем*. Листинг 5-1 описывает структуру для хранения информации об учётной записи пользователя:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Листинг 5-1: определение структуры <code>User</code></span>

После определения структуры можно создавать её *экземпляр*, назначая определённое значение каждому полю с соответствующим типом данных. Чтобы создать экземпляр, мы указываем имя структуры, затем добавляем фигурные скобки и включаем в них пары `ключ: значение` (key: value), где ключами являются имена полей, а значениями являются данные, которые мы хотим сохранить в полях. Нет необходимости чётко следовать порядку объявления полей в описании структуры (но всё-таки желательно для удобства чтения). Другими словами, объявление структуры - это как шаблон нашего типа, в то время как экземпляр структуры использует этот шаблон, заполняя его определёнными данными, для создания значений нашего типа. Например, можно объявить пользователя как в листинге 5-2:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Listing 5-2: Creating an instance of the <code>User</code> struct</span>

To get a specific value from a struct, we use dot notation. If we wanted just this user’s email address, we could use `user1.email` wherever we wanted to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Listing 5-3 shows how to change the value in the `email` field of a mutable `User` instance.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Listing 5-3: Changing the value in the <code>email</code> field of a <code>User</code> instance</span>

Заметим, что весь экземпляр структуры должен быть изменяемым; Rust не позволяет помечать изменяемыми отдельные поля. Как и для любого другого выражения, мы можем использовать выражение создания структуры в качестве последнего выражения тела функции для неявного возврата нового экземпляра.

На листинге 5-4 функция `build_user` возвращает экземпляр `User` с указанным адресом и именем. Поле `active` получает значение `true`, а поле `sign_in_count` получает значение `1`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Listing 5-4: A <code>build_user</code> function that takes an email and username and returns a <code>User</code> instance</span>

Имеет смысл называть параметры функции теми же именами, что и поля структуры, но необходимость повторять `email` и `username` для названий полей и переменных несколько утомительна. Если структура имеет много полей,  повторение каждого имени станет ещё более раздражающим. К счастью, есть удобное сокращение!

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Использование сокращённой инициализации поля

Так как имена входных параметров функции и полей структуры являются полностью идентичными в листинге 5-4, возможно использовать синтаксис *сокращённой инициализации поля*, чтобы переписать `build_user` так, чтобы он работал точно также, но не содержал повторений для `email` и `username`, как в листинге 5-5.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Листинг 5-5: Функция <code>build_user</code>, использующая сокращённую инициализацию поля, когда параметры <code>email</code> и <code>username</code> имеют те же имена, что и поля struct</span>

Здесь происходит создание нового экземпляра структуры `User`, которая имеет поле с именем `email`. Мы хотим установить поле структуры `email` значением входного параметра `email` функции `build_user`. Так как поле `email` и входной параметр функции `email` имеют одинаковое название, можно писать просто `email` вместо кода `email: email`.

### Создание экземпляра структуры из экземпляра другой структуры с помощью синтаксиса обновления структуры

It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using *struct update syntax*.

Сначала в листинге 5-6 показано, как обычно создаётся новый экземпляр `User` в `user2` без синтаксиса обновления. Мы задаём новое значение для `email`, но в остальном используем те же значения из `user1`, которые были заданы в листинге 5-2.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Листинг 5-6: Создание нового экземпляра <code>User</code> с использованием одного из значений из экземпляра <code>user1</code></span>

Используя синтаксис обновления структуры, можно получить тот же эффект, используя меньше кода как показано в листинге 5-7. Синтаксис `..` указывает, что оставшиеся поля устанавливаются неявно и должны иметь значения из указанного экземпляра.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Листинг 5-7: Использование синтаксиса обновления структуры для установки нового значения <code>email</code> для экземпляра <code>User</code>, но использование остальных значений из экземпляра <code>user1</code></span>

Код в листинге 5-7 также создаёт экземпляр в `user2`, который имеет другое значение для `email`, но с тем же значением для полей `username`, `active` и `sign_in_count` из `user1`. Оператор `..user1` должен стоять последним для указания на получение значений всех оставшихся полей из соответствующих полей в `user1`, но можно указать значения для любого количества полей в любом порядке, независимо от порядка полей в определении структуры.

Заметим, что синтаксис обновления структуры использует `=` как присваивание. Это связано с перемещением данных, как мы видели в разделе ["Способы взаимодействия переменных и данных: перемещение"](ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move). В этом примере мы больше не можем использовать `user1` после создания `user2`, потому что `String` в поле `username` из `user1` было перемещено в `user2`. Если бы мы задали `user2` новые значения `String` для `email` и `username`, и при этом использовать только значения `active` и `sign_in_count` из `user1`, то `user1` все ещё будет действительным после создания `user2`. Типы `active` и `sign_in_count` являются типами, реализующими типаж `Copy`, поэтому будет применяться поведение, о котором мы говорили в разделе ["Stack-Only Data: Copy"](ch04-01-what-is-ownership.html#stack-only-data-copy).

### Кортежные структуры: структуры без именованных полей для создания разных типов

Rust также поддерживает структуры, похожие на кортежи, которые называются *кортежные структуры*. Кортежные структуры обладают дополнительным смыслом, который даёт имя структуры, но при этом не имеют имён, связанных с их полями. Скорее, они просто хранят типы полей. Кортежные структуры полезны, когда вы хотите дать имя всему кортежу и сделать кортеж отличным от других кортежей, и когда именование каждого поля, как в обычной структуре, было бы многословным или избыточным.

To define a tuple struct, start with the `struct` keyword and the struct name followed by the types in the tuple. For example, here we define and use two tuple structs named `Color` and `Point`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

Заметим, что значения `black` и `origin` являются разными типами, потому что это экземпляры разных кортежных структур. Каждая определённая вами структура имеет свой собственный тип, даже если поля внутри структуры имеют одинаковые типы. Например, функция, принимающая параметр типа `Color`, не сможет принять в качестве аргумента `Point`, даже если оба типа состоят из трёх значений `i32`. В остальном экземпляры кортежных структур ведут себя как кортежи: вы можете деструктурировать их на отдельные части, вы можете использовать `.` с последующим индексом для доступа к отдельному значению, и так далее.

### Единично-подобные структуры: структуры без полей

Также можно определять структуры, не имеющие полей! Они называются *единично-подобными структурами*, поскольку ведут себя аналогично `()`, единичному типу, о котором мы говорили в разделе ["Тип кортежа"](ch03-02-data-types.html#the-tuple-type). Единично-подобные структуры могут быть полезны, когда требуется реализовать типаж для некоторого типа, но у вас нет данных, которые нужно хранить в самом типе. Мы обсудим типажи в главе 10. Вот пример объявления и создание экземпляра единичной структуры с именем `AlwaysEqual`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

To define `AlwaysEqual`, we use the `struct` keyword, the name we want, then a semicolon. No need for curly brackets or parentheses! Then we can get an instance of `AlwaysEqual` in the `subject` variable in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine that later we’ll implement behavior for this type such that every instance of `AlwaysEqual` is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.

> ### Владение данными структуры
>
> In the `User` struct definition in Listing 5-1, we used the owned `String` type rather than the `&str` string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
>
> It’s also possible for structs to store references to data owned by something else, but to do so requires the use of *lifetimes*, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:
>
> <span class="filename">Файл : src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> Компилятор будет жаловаться на необходимость определения времени жизни ссылок:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs, but for now, we’ll fix errors like these using owned types like `String` instead of references like `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->
