## Синтаксис метода

*Methods* are similar to functions: they’re declared with the `fn` keyword and
their name, they can have parameters and a return value, and they contain some
code that is run when they’re called from somewhere else. However, methods are
different from functions in that they’re defined within the context of a struct
(or an enum or a trait object, which we cover in Chapters 6 and 17,
respectively), and their first parameter is always `self`, which represents the
instance of the struct the method is being called on.

### Определение методов

Давайте изменим функцию `area` так, чтобы она имела экземпляр `Rectangle` в качестве параметра и сделаем ее методом `area`, определенным для структуры `Rectangle`, как показано в листинге 5-13:

<span class="filename">Файл: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

<span class="caption">Пример 5-13: Определение метода <code>area</code> в структуре <code>Rectangle</code></span>

Для определения функции в контексте типа `Rectangle`, мы начинаем блок `impl` (implementation - реализация). Затем переносим функцию `area` внутрь фигурных скобок `impl` и меняем первый (в данном случае единственный) параметр в сигнатуре на `self`, и далее везде в теле метода. В `main`, там где мы вызывали функцию `area` и передавали ей переменную `rect1` в качестве аргумента, теперь можно использовать *синтаксис метода* для вызова метода `area` на экземпляре типа `Rectangle`. Синтаксис метода идет после экземпляра: мы добавляем точечную нотацию за которой следует название метода, круглые скобки и любые аргументы.

In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`
because Rust knows the type of `self` is `Rectangle` due to this method’s being
inside the `impl Rectangle` context. Note that we still need to use the `&`
before `self`, just as we did in `&Rectangle`. Methods can take ownership of
`self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably,
just as they can any other parameter.

We’ve chosen `&self` here for the same reason we used `&Rectangle` in the
function version: we don’t want to take ownership, and we just want to read the
data in the struct, not write to it. If we wanted to change the instance that
we’ve called the method on as part of what the method does, we’d use `&mut self` as the first parameter. Having a method that takes ownership of the
instance by using just `self` as the first parameter is rare; this technique is
usually used when the method transforms `self` into something else and you want
to prevent the caller from using the original instance after the transformation.

Основным преимуществом использования методов вместо функций, в дополнение к использованию синтаксиса метода, где не нужно повторять тип `self` в каждой сигнатуре метода, является организация кода. Мы собрали все, что мы можем сделать с экземпляром типа в одном блоке `impl`, не заставляя будущих пользователей нашего кода искать предоставленные возможности `Rectangle` в разных местах библиотеки.

> ### Где используется оператор `->`?
> В языках C++, используются два различных оператора для вызова методов: используется `.`, если вызывается метод непосредственно у экземпляра структуры и используется `->`, если вызывается метод из ссылки на объект. Другими словами, если `object` является ссылкой, то вызовы метода `object->something()` и ` (*object).something()` являются аналогичными.
> Rust не имет такого эквивалента (оператора `->`), наоборот, в Rust есть функциональность,
> которая называется *автоматическая ссылка и разыменование*. Вызов методов является
> одним из немногих мест в Rust, в котором есть такая функциональность.
> Вот как это работает: когда вы вызываете метод `object.something()`, Rust автоматически
> добавляет `&`, `&mut` или  `*`, таким образом, чтобы `object` соответствовал сигнатуре метода. Другими словами, следующий код является одинаковым:
> ```rust
> #[derive(Debug,Copy,Clone)]
> struct Point {
>    x: f64,
>    y: f64,
> }
> 
> impl Point {
>    fn distance(&self, other: &Point) -> f64 {
>       let x_squared = f64::powi(other.x - self.x, 2);
>       let y_squared = f64::powi(other.y - self.y, 2);
> 
>       f64::sqrt(x_squared + y_squared)
>    }
> }
> 
> fn main() {
>    let p1 = Point { x: 0.0, y: 0.0 };
>    let p2 = Point { x: 5.0, y: 6.5 };
>    p1.distance(&p2);
>    (&p1).distance(&p2);
> }
> ```
> The first one looks much cleaner. This automatic referencing behavior works
> because methods have a clear receiver—the type of `self`. Given the receiver
> and name of a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact
> that Rust makes borrowing implicit for method receivers is a big part of
> making ownership ergonomic in practice.

### Методы с несколькими параметрами

Давайте попробуем использование методов, реализовав второй метод у структуры `Rectangle`. На этот раз мы хотим, чтобы экземпляр `Rectangle` использовал другой экземпляр типа `Rectangle` и возвращал `true`, если второй `Rectangle` может полностью разместиться внутри площади экземпляра `self` ; в противном случае он должен возвращать `false` . То есть мы хотим иметь возможность написать программу, показанную в листинге 5-14, в которой определили метод `can_hold`.

<span class="filename">Файл: src/main.rs</span>

```rust,ignore
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

<span class="caption">Листинг 5-14: Использование еще не написанного метода <code>can_hold</code></span>

И ожидаемый результат будет выглядеть следующим образом, потому что оба значения в экземпляре `rect2` меньше, чем размеры в экземпляре `rect1`, то `rect3` шире, чем `rect1` :

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we’d need a mutable borrow),
and we want `main` to retain ownership of `rect2` so we can use it again after
calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are both greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.

<span class="filename">Файл: src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

<span class="caption">Листинг 5-15: Реализация метода <code>can_hold</code> для структуры<code>Rectangle</code>, который принимает другой экземпляр <code>Rectangle</code> в качестве параметра</span>

Когда мы запустим код с функцией `main` листинга 5-14, мы получим желаемый вывод. Методы могут принимать несколько параметров, которые мы добавляем в сигнатуру после первого  параметра `self` , и эти параметры работают так же, как параметры в функциях.

### Ассоциированные функции

Another useful feature of `impl` blocks is that we’re allowed to define
functions within `impl` blocks that *don’t* take `self` as a parameter. These
are called *associated functions* because they’re associated with the struct.
They’re still functions, not methods, because they don’t have an instance of
the struct to work with. You’ve already used the `String::from` associated
function.

Ассоциированные функции часто используются в качестве конструкторов, которые будут возвращать новый экземпляр структуры. Например, мы могли бы предоставить ассоциированную функцию, которая будет иметь один параметр измерения и использовать его как ширину и высоту, тем самым облегчая создание квадратного `Rectangle` прямоугольника, вместо указания одно и того же значение дважды:

<span class="filename">Файл: src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in Chapter 7.

### Несколько блоков `impl`

Для каждой структуры разрешено иметь множество `impl` блоков. Например, листинг 5-15 является эквивалентным коду из листинга 5-16, который описывает метод в своем отдельном блоке `impl`.

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

<span class="caption">Листинг 5-16: Переписанный листинг 5-15 с использованием нескольких блоков <code>impl</code></span>

There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.

## Итоги

Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. Methods let you specify the
behavior that instances of your structs have, and associated functions let you
namespace functionality that is particular to your struct without having an
instance available.

But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.
