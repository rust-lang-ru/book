## Расширенные типы

Система типов Rust имеет некоторые возможности, которые мы упоминали в этой книге, но ещё не обсуждали. Мы начнём с обсуждения новых типов (newtypes) в целом, по мере изучения того, почему новые типы полезны в качестве типов. Затем мы перейдём к псевдонимам, возможности похожей на новые типы (newtypes), но с немного другой семантикой. Мы также обсудим тип `!` и с динамическими типами (dynamically sized type).

### Использование Newtype шаблона для безопасности типов и реализации абстракций

> Note: This section assumes you’ve read the earlier section [“Using the Newtype Pattern to Implement External Traits on External Types.”](ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types)<!-- ignore -->

Шаблон newtype полезен для задач помимо тех, которые мы обсуждали до сих пор, включая статическое обеспечение того, чтобы значения никогда не путались и указывали единицы значения. Вы видели пример использования newtype для обозначения единиц в листинге 19-15. Вспомним, что структуры `Millimeters` и `Meters` содержат обёрнутые значения `u32` в newtype. Если бы мы написали функцию с параметром типа `Millimeters`, мы не смогли бы скомпилировать программу, которая случайно пыталась вызвать эту функция со значением типа `Meters` или обычным `u32`.

Другое использование шаблона newtype - абстрагирование от некоторых деталей реализации типа: новый тип может предоставлять открытый API, отличный от API приватного внутреннего типа, если мы  напрямую использовали новый тип для ограничения доступного функционала, например.

Newtypes can also hide internal implementation. For example, we could provide a `People` type to wrap a `HashMap<i32, String>` that stores a person’s ID associated with their name. Code using `People` would only interact with the public API we provide, such as a method to add a name string to the `People` collection; that code wouldn’t need to know that we assign an `i32` ID to names internally. The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details, which we discussed in the [“Encapsulation that Hides Implementation Details”](ch17-01-what-is-oo.html#encapsulation-that-hides-implementation-details)<!-- ignore --> section of Chapter 17.

### Создание синонимов типа с помощью псевдонимов типа

Наряду с шаблоном newtype, Rust предоставляет возможность объявить *псевдоним типа* чтобы дать существующему типу другое имя. Для этого мы используем ключевое слово `type`. Например, мы можем создать псевдоним типа `Kilometers` для `i32` следующим образом:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Теперь псевдоним `Kilometers` является *синонимом* для `i32`; в отличие от типов `Millimeters` и `Meters`, которые мы создали в листинге 19-15, `Kilometers` не являются отдельными, новыми типами. Значения с типом `Kilometers` будут обрабатываться так же, как значения типа `i32`:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

Because `Kilometers` and `i32` are the same type, we can add values of both types and we can pass `Kilometers` values to functions that take `i32` parameters. However, using this method, we don’t get the type checking benefits that we get from the newtype pattern discussed earlier.

Синонимы в основном используются для уменьшения повторяемости. Например, мы у нас есть тип:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Запись этого длинного типа в сигнатурах функций и в виде аннотаций типов по всему коду может быть утомительной и приводить к ошибкам. Представьте, что у вас есть проект, полный кодом как в листинге 19-24.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-24/src/main.rs:here}}
```

<span class="caption">Листинг 19-24: Использование длинного типа во многих местах</span>

A type alias makes this code more manageable by reducing the repetition. In Listing 19-25, we’ve introduced an alias named `Thunk` for the verbose type and can replace all uses of the type with the shorter alias `Thunk`.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-25/src/main.rs:here}}
```

<span class="caption">Listing 19-25: Introducing a type alias <code>Thunk</code> to reduce repetition</span>

This code is much easier to read and write! Choosing a meaningful name for a type alias can help communicate your intent as well (*thunk* is a word for code to be evaluated at a later time, so it’s an appropriate name for a closure that gets stored).

Type aliases are also commonly used with the `Result<T, E>` type for reducing repetition. Consider the `std::io` module in the standard library. I/O operations often return a `Result<T, E>` to handle situations when operations fail to work. This library has a `std::io::Error` struct that represents all possible I/O errors. Many of the functions in `std::io` will be returning `Result<T, E>` where the `E` is `std::io::Error`, such as these functions in the `Write` trait:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type alias declaration:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Поскольку это объявление находится в модуле `std::io`, мы можем использовать полностью квалифицированный псевдоним `std::io::Result<T>`, что является `Result<T, E>` с типом `E` заполненным типом `std::io::Error`. Сигнатуры функций типажа `Write` в конечном итоге выглядят как:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

Псевдоним типа помогает двумя способами: он облегчает написание кода *и* даёт нам согласованный интерфейс для всего из `std::io`. Поскольку это псевдоним, то это просто ещё один тип `Result<T, E>`, что означает, что с ним мы можем использовать любые методы, которые работают с `Result<T, E>`, а также специальный синтаксис вроде `?` оператора.

### Тип Never, который никогда не возвращается

Rust has a special type named `!` that’s known in type theory lingo as the *empty type* because it has no values. We prefer to call it the *never type* because it stands in the place of the return type when a function will never return. Here is an example:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Этот код читается как «функция `bar` никогда не возвращается». Функции, которые никогда не возвращаются называются *расходящимися функциями* (diverging functions). Нельзя создавать значения типа `!`, так как `bar` никогда не может вернуться.

Но для чего нужен тип, для которого вы никогда не сможете создать значения? Напомним код из листинга 2-5; мы воспроизвели его часть здесь в листинге 19-26.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

<span class="caption">Листинг 19-26: Сопоставление <code>match</code> с веткой, которая заканчивается <code>continue</code></span>

В то время мы опустили некоторые детали в этом коде. В главе 6 раздела ["Оператор управления потоком `match`"](ch06-02-match.html#the-match-control-flow-operator)<!--  --> мы обсуждали, что все ветви `match` должны возвращать одинаковый тип. Например, следующий код не работает:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

The type of `guess` in this code would have to be an integer *and* a string, and Rust requires that `guess` have only one type. So what does `continue` return? How were we allowed to return a `u32` from one arm and have another arm that ends with `continue` in Listing 19-26?

Как вы уже возможно догадались, `continue` имеет  значение `!`. То есть, когда Rust вычисляет тип `guess`, он смотрит на обе сопоставляемые ветки, первая со значением `u32` и последняя со значением `!`. Так как `!` никогда не может иметь значение, то Rust решает что типом `guess` является тип `u32`.

Формальным способом описания этого поведения является то, что выражения типа `!` могу быть приведены (coerced) к любому другому типу. Нам разрешено закончить сопоставление этой `match` ветки с помощью `continue`, потому что `continue` не возвращает значение; вместо этого она передаёт контроль обратно в начало цикла, поэтому в случае `Err` мы никогда не присваиваем `guess` значение.

The never type is useful with the `panic!` macro as well. Remember the `unwrap` function that we call on `Option<T>` values to produce a value or panic? Here is its definition:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

In this code, the same thing happens as in the `match` in Listing 19-26: Rust sees that `val` has the type `T` and `panic!` has the type `!`, so the result of the overall `match` expression is `T`. This code works because `panic!` doesn’t produce a value; it ends the program. In the `None` case, we won’t be returning a value from `unwrap`, so this code is valid.

Последнее выражение, которое имеет тип `!` это `loop`:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Here, the loop never ends, so `!` is the value of the expression. However, this wouldn’t be true if we included a `break`, because the loop would terminate when it got to the `break`.

### Динамические типы и `Sized` типаж

В связи с необходимостью Rust знать определённые детали, например, сколько места выделять для значения определённого типа, то существует краеугольный камень его системы типов, который может сбивать с толку. Это концепция *динамических типов* (dynamically sized types). Иногда она упоминается как *DST* или *безразмерные типы* (unsized types), эти типы позволяют писать код, используя значения, чей размер известен только во время выполнения.

Давайте углубимся в детали динамического типа `str`, который мы использовали на протяжении всей книги. Все верно, не типа `&str`, а типа `str` самого по себе, который является DST. Мы не можем знать, какой длины строка до момента времени выполнения, то есть мы не можем создать переменную типа `str` и не можем принять аргумент типа `str`. Рассмотрим следующий код, который не работает:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust должен знать, сколько памяти выделить для любого значения конкретного типа и все значения типа должны использовать одинаковый объем памяти. Если Rust позволил бы нам написать такой код, то эти два значения `str` должны были бы занимать одинаковое количество памяти. Но они имеют разную длину: `s1` нужно 12 байтов памяти, а для `s2` нужно 15. Вот почему невозможно создать переменную имеющую динамический тип.

So what do we do? In this case, you already know the answer: we make the types of `s1` and `s2` a `&str` rather than a `str`. Recall that in the [“String Slices”](ch04-03-slices.html#string-slices)<!-- ignore --> section of Chapter 4, we said the slice data structure stores the starting position and the length of the slice.

So although a `&T` is a single value that stores the memory address of where the `T` is located, a `&str` is *two* values: the address of the `str` and its length. As such, we can know the size of a `&str` value at compile time: it’s twice the length of a `usize`. That is, we always know the size of a `&str`, no matter how long the string it refers to is. In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

We can combine `str` with all kinds of pointers: for example, `Box<str>` or `Rc<str>`. In fact, you’ve seen this before but with a different dynamically sized type: traits. Every trait is a dynamically sized type we can refer to by using the name of the trait. In Chapter 17 in the [“Using Trait Objects That Allow for Values of Different Types”](ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)<!-- ignore --> section, we mentioned that to use traits as trait objects, we must put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>` (`Rc<dyn Trait>` would work too).

To work with DSTs, Rust has a particular trait called the `Sized` trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on `Sized` to every generic function. That is, a generic function definition like this:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

на самом деле рассматривается как если бы мы написали её в виде:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

По умолчанию обобщённые функции будут работать только с типами чей размер известен в время компиляции. Тем не менее, можно использовать следующий специальный синтаксис, чтобы ослабить это ограничение:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

A trait bound on `?Sized` means “`T` may or may not be `Sized`” and this notation overrides the default that generic types must have a known size at compile time. The `?Trait` syntax with this meaning is only available for `Sized`, not any other traits.

Также обратите внимание, что мы поменяли тип параметра `t` с `T` на `&T`. Поскольку тип мог бы не быть `Sized`, мы должны использовать его за каким-либо указателем. В в этом случае мы выбрали ссылку.

Далее мы поговорим о функциях и замыканиях!
