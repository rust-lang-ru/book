## Пример использования структур

Для понимания, где мы могли бы использовать структуры, мы напишем программу, которая будет рассчитывать площадь прямоугольника. Мы начнём с создания переменных, а потом постепенно напишем код, который будет использовать структуры.

Создадим проект программы с помощью Cargo с именем *rectangles*, которая будет получать на вход длину и ширину прямоугольника в пикселях и будет рассчитывать площадь прямоугольника. Листинг 5-8 показывает короткую программу с одним из способов сделать именно это, код в файле проекта {em1}src/main.rs{/em1} .

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
```

<span class="caption">Пример 5-8: Расчёт площади треугольника с помощью отдельных переменных ширины и высоты</span>

Теперь, проверим её работу `cargo run`:

```text
The area of the rectangle is 1500 square pixels.
```

Хотя пример 5-8 работает и расчитывает площадь прямоугольника, мы можем улучшить
программу. Переменные длина и ширина связаны логически, т.к. они описывают параметры
прямоугольника.

Проблема данного метода очевидна из сигнатуры `area`:

```rust,ignore
fn area(length: u32, width: u32) -> u32 {
```

Функция `area` должна расчитывать площадь одного прямоугольника, но у функции описано два параметра. Эти параметры связаны, но это никак не отражено в коде программы. Код был бы более очевидным и управляемым, если бы переменные ширины и длины были сгруппированы вместе. Мы уже знаем один способ группировки переменных из раздела  {a1}“Тип кортеж”{/comment2}{/a1}{comment2} главы 3, используя кортежи.

### Изменение с помощью кортежей

Листинг 5-9 это другая версия программы, используемая кортежи.

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

<span class="caption">Listing 5-9: Specifying the width and height of the
rectangle with a tuple</span>

In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way, this version is less
clear: tuples don’t name their elements, so our calculation has become more
confusing because we have to index into the parts of the tuple.

It doesn’t matter if we mix up width and height for the area calculation, but
if we want to draw the rectangle on the screen, it would matter! We would have
to keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. If someone else worked on this code, they would have to figure this
out and keep it in mind as well. It would be easy to forget or mix up these
values and cause errors, because we haven’t conveyed the meaning of our data in
our code.

### Изменения для структур: добавим больше смысла

We use structs to add meaning by labeling the data. We can transform the tuple
we’re using into a data type with a name for the whole as well as names for the
parts, as shown in Listing 5-10.

<span class="filename">Файл: src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

<span class="caption">Пример 5-10: Определение структуры <code>Rectangle</code></span>

Here we’ve defined a struct and named it `Rectangle`. Inside the curly
brackets, we defined the fields as `width` and `height`, both of which have
type `u32`. Then in `main`, we created a particular instance of `Rectangle`
that has a width of 30 and a height of 50.

Our `area` function is now defined with one parameter, which we’ve named
`rectangle`, whose type is an immutable borrow of a struct `Rectangle`
instance. As mentioned in Chapter 4, we want to borrow the struct rather than
take ownership of it. This way, `main` retains its ownership and can continue
using `rect1`, which is the reason we use the `&` in the function signature and
where we call the function.

The `area` function accesses the `width` and `height` fields of the `Rectangle`
instance. Our function signature for `area` now says exactly what we mean:
calculate the area of `Rectangle`, using its `width` and `height` fields. This
conveys that the width and height are related to each other, and it gives
descriptive names to the values rather than using the tuple index values of `0`
and `1`. This is a win for clarity.

### Добавление полезной функциональности используя выводимые типажи

Было бы не плохо иметь возможность печатать экземпляр `Rectangle` во время отладки программы и видеть значения всех полей. Листинг 5-11 использует макрос `println!`, который мы уже использовали в предыдущих главах. Тем не менее, это не работает.

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

<span class="caption">Листинг 5-11: Попытка печатать экземпляр <code>Rectangle</code></span>

При компиляции этого кода мы получаем ошибку с сообщением:

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

Макрос `println!` умеет делать разные виды форматирования по умолчанию, фигурные скобки `println!` используются для форматирования, известного как типаж `Display`: который предназначен для использования пользователем. Примитивные типы изученные ранее, по умолчанию реализуют `Display`, потому что есть только один способ отобразить число `1` или любой другой примитивный тип пользователю. Но для структур у которых `println!` должен форматировать способ вывода данных, это является менее очевидным, потому что есть гораздо больше возможностей для отображения: Вы хотите запятые или нет? Вы хотите печатать фигурные скобки? Нужно ли показать все поля? Из-за этой неоднозначности Rust не пытается  угадать, что нам нужно и структуры не имеют готовой реализации типажа `Display`.

Продолжив чтение текста ошибки, мы найдем полезное замечание:

```text
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Давайте попробуем! Вызов макроса `println!` теперь будет выглядеть так `println!("rect1 is {:?}", rect1);`. Ввод спецификатора `:? ` внутри фигурных скобок говорит макросу `println!`, что мы хотим использовать формат вывода называемый `Debug`. Типаж `Debug` позволяет печатать структуру способом, удобным для разработчиков, чтобы видеть значение во время отладки кода.

Скомпилируем код с этими изменениями. Упс! Вы все еще получаем ошибку:

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
```

Снова компилятор дает нам полезное замечание:

```text
= help: the trait `std::fmt::Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
```

Rust *НЕ* включает функциональность для печати отладочной информации, мы должны явно включить эту функциональность для нашей структуры. Чтобы это сделать, добавляем аннотацию `#[derive(Debug)]` сразу перед определением структуры как показано в листинге 5-12.

<span class="filename">Файл: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

<span class="caption">Listing 5-12: Adding the annotation to derive the <code>Debug</code>
trait and printing the <code>Rectangle</code> instance using debug formatting</span>

Теперь при запуске программы не получаем ошибок и увидим следующий вывод:

```text
rect1 is Rectangle { width: 30, height: 50 }
```

Отлично! Это не делает вывод приятнее, но показывает значения всех полей экземпляра, которые определенно помогут при отладке. Когда у нас структуры больше, то полезно иметь более простой для чтения вывод; в таком случае можно использовать код `{:#?}` вместо ` {:?}` в строке макроса `println!`. При использовании стиля `{:#?}` в примере вывод будет выглядеть так:

```text
rect1 is Rectangle {
    width: 30,
    height: 50
}
```

Rust предоставляет несколько типажей для использования в аннотации `derive`, они умеют давать полезное поведение пользовательским типам. Эти типажи и их поведение перечислены в приложении C. Мы обсудим как реализовать данные типажи с пользовательским поведением, а также как создавать свои собственные типажи в главе 10.

Функция `area` является довольно специфичной: она считает только площадь прямоугольников. Было бы полезно привязать данное поведение как можно ближе к структуре `Rectangle`, потому что код не будет работать с любым другим типом. Давайте рассмотрим, как можно продолжить изменения этого кода превращая функцию `area` в  `area` *метод* определенный для типа `Rectangle`.
