## Синтаксис метода

*Методы* похожи на функции: они объявлены с помощью ключевого слова `fn` и его имени. Они могут иметь параметры и возвращаемое значение, могут содержать некоторый код, который выполняется при вызове из другого места. Тем не менее, методы отличаются от функций тем, что они определены внутри контекста структуры (также перечисления или объекта-типажа, которые мы рассмотрим в главе 6 и 17, соответственно), а их первым параметром всегда является `self`, который представляет экземпляр структуры для которого этот метод будет вызван.

### Определение методов

Давайте изменим функцию `area` так, чтобы она имела экземпляр `Rectangle` в качестве входного параметра и сделаем её методом `area`, определённым для структуры `Rectangle`, как показано в листинге 5-13:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Листинг 5-13: Определение метода <code>area</code> у структуры <code>Rectangle</code></span>

Для определения функции в контексте типа `Rectangle`, мы начинаем блок `impl` (implementation - реализация). Затем переносим функцию `area` внутрь фигурных скобок `impl` и меняем первый (в данном случае единственный) параметр в сигнатуре на `self`, и далее везде в теле метода. В `main`, там где мы вызывали функцию `area` и передавали ей переменную `rect1` в качестве аргумента, теперь можно использовать *синтаксис метода* для вызова метода `area` на экземпляре типа `Rectangle`. Синтаксис метода идёт после экземпляра: мы добавляем точечную нотацию за которой следует название метода, круглые скобки и любые аргументы.

В сигнатуре `area`, используется `&self` вместо `rectangle: &Rectangle` потому что Rust знает, что тип `self` является типом `Rectangle`, так как данный метод находится внутри `impl Rectangle` контекста. Заметьте, всё ещё нужно использовать `&` перед `self`, как мы делали `&Rectangle`. Методы могут принимать во владение `self`, заимствовать неизменный `self`, как делалось ранее или заимствовать изменяемый `self`, как любые другие параметры.

We’ve chosen `&self` here for the same reason we used `&Rectangle` in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use `&mut self` as the first parameter. Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.

The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of `self` in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one `impl` block rather than making future users of our code search for capabilities of `Rectangle` in various places in the library we provide.

> ### Где используется оператор `->`?
> В языках C++, используются два различных оператора для вызова методов: используется `.`, если вызывается метод непосредственно у экземпляра структуры и используется `->`, если вызывается метод у ссылки на объект. Другими словами, если `object` является ссылкой, то вызовы метода `object->something()` и ` (*object).something()` являются аналогичными.
> Rust не имеет эквивалента оператора `->`, наоборот в Rust есть функциональность называемая *автоматическое обращение по ссылке и разыменование* (automatic referencing and dereferencing). Вызов методов является одним из немногих мест в Rust, в котором есть такое поведение.
> Вот как это работает: когда вы вызываете метод `object.something()`, Rust автоматически добавляет `&`, `&mut` или  `*`, таким образом, чтобы `object` соответствовал сигнатуре метода. Другими словами, следующий код является одинаковым:

<!-- CAN'T EXTRACT SEE BUG TODO -->

> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
> The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of `self`. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

### Методы с несколькими параметрами

Let’s practice using methods by implementing a second method on the `Rectangle` struct. This time, we want an instance of `Rectangle` to take another instance of `Rectangle` and return `true` if the second `Rectangle` can fit completely within `self`; otherwise it should return `false`. That is, we want to be able to write the program shown in Listing 5-14, once we’ve defined the `can_hold` method.

<span class="filename">Файл: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Листинг 5-14: Использование ещё не написанного метода <code>can_hold</code></span>

And the expected output would look like the following, because both dimensions of `rect2` are smaller than the dimensions of `rect1` but `rect3` is wider than `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle` block. The method name will be `can_hold`, and it will take an immutable borrow of another `Rectangle` as a parameter. We can tell what the type of the parameter will be by looking at the code that calls the method: `rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to `rect2`, an instance of `Rectangle`. This makes sense because we only need to read `rect2` (rather than write, which would mean we’d need a mutable borrow), and we want `main` to retain ownership of `rect2` so we can use it again after calling the `can_hold` method. The return value of `can_hold` will be a Boolean, and the implementation will check whether the width and height of `self` are both greater than the width and height of the other `Rectangle`, respectively. Let’s add the new `can_hold` method to the `impl` block from Listing 5-13, shown in Listing 5-15.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Листинг 5-15: реализация метода <code>can_hold</code> у структуры <code>Rectangle</code>, который принимает другой экземпляр <code>Rectangle</code> в качестве параметра</span>

When we run this code with the `main` function in Listing 5-14, we’ll get our desired output. Methods can take multiple parameters that we add to the signature after the `self` parameter, and those parameters work just like parameters in functions.

### Ассоциированные функции

Another useful feature of `impl` blocks is that we’re allowed to define functions within `impl` blocks that *don’t* take `self` as a parameter. These are called *associated functions* because they’re associated with the struct. They’re still functions, not methods, because they don’t have an instance of the struct to work with. You’ve already used the `String::from` associated function.

Associated functions are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square `Rectangle` rather than having to specify the same value twice:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

To call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);` is an example. This function is namespaced by the struct: the `::` syntax is used for both associated functions and namespaces created by modules. We’ll discuss modules in Chapter 7.

### Несколько блоков `impl`

Для каждой структуры разрешено иметь множество `impl` блоков. Например, листинг 5-15 является эквивалентным коду из листинга 5-16, который описывает метод в своём отдельном блоке `impl`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Листинг 5-16: Переписанный листинг 5-15 с использованием нескольких блоков <code>impl</code></span>

There’s no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax. We’ll see a case in which multiple `impl` blocks are useful in Chapter 10, where we discuss generic types and traits.

## Итоги

Структуры позволяют создавать собственные типы, которые имеют смысл в вашей предметной области. Используя структуры, вы храните ассоциированные друг с другом фрагменты данных и даёте название частям данных, чтобы ваш код был более понятным. Методы позволяют определить поведение, которое имеют экземпляры ваших структур, а ассоциированные функции позволяют привязать функциональность к вашей структуре, не обращаясь к её экземпляру.

But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.
