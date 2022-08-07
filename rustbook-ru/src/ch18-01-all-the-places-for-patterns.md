## Все случаи, где могут быть использованы шаблоны

В процессе использования языка Rust вы часто используете шаблоны, даже не осознавая этого! В этом разделе обсуждаются все случаи, где использование шаблонов является корректным.

### Ветки `match`

Как обсуждалось в главе 6, мы используем шаблоны в ветках выражений `match`. Формально выражения `match` определяется как ключевое слово `match`, значение используемое для сопоставления, одна или несколько веток, которые состоят из шаблона и выражения для выполнения, если значение соответствует шаблону этой ветки, как здесь:

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

For example, here's the `match` expression from Listing 6-5 that matches on an `Option<i32>` value in the variable `x`:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

The patterns in this `match` expression are the `None` and `Some(i)` on the left of each arrow.

Одно из требований к выражениям `match` состоит в том, что они должны быть *исчерпывающими* (exhaustive) в том смысле, что они должны учитывать все возможности для значения в выражении `match`. Один из способов убедиться, что вы рассмотрели каждую возможность - это иметь шаблон перехвата всех вариантов в последней ветке выражения: например, имя переменной, совпадающее с любым значением, никогда не может потерпеть неудачу и таким образом, охватывает каждый оставшийся случай.

The particular pattern `_` will match anything, but it never binds to a variable, so it’s often used in the last match arm. The `_` pattern can be useful when you want to ignore any value not specified, for example. We’ll cover the `_` pattern in more detail in the [“Ignoring Values in a Pattern”](ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern)<!-- ignore --> section later in this chapter.

### Условные выражения `if let`

В главе 6 мы обсуждали, как использовать выражения `if let` как правило в качестве более короткого способа записи эквивалента `match`, которое обрабатывает только один случай. Дополнительно `if let` может иметь соответствующий `else`, содержащий код для выполнения, если шаблон выражения `if let` не совпадает.

Listing 18-1 shows that it’s also possible to mix and match `if let`, `else if`, and `else if let` expressions. Doing so gives us more flexibility than a `match` expression in which we can express only one value to compare with the patterns. Also, Rust doesn't require that the conditions in a series of `if let`, `else if`, `else if let` arms relate to each other.

The code in Listing 18-1 determines what color to make your background based on a series of checks for several conditions. For this example, we’ve created variables with hardcoded values that a real program might receive from user input.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-01/src/main.rs}}
```

<span class="caption">Листинг 18-1: Использование условных конструкций <code>if let</code>, <code>else if</code>, <code>else if let</code>, и <code>else</code></span>

If the user specifies a favorite color, that color is used as the background. If no favorite color is specified and today is Tuesday, the background color is green. Otherwise, if the user specifies their age as a string and we can parse it as a number successfully, the color is either purple or orange depending on the value of the number. If none of these conditions apply, the background color is blue.

Эта условная структура позволяет поддерживать сложные требования. С жёстко закодированными значениями, которые у нас здесь есть, этот пример напечатает `Using purple as the background color`.

Можно увидеть, что `if let` может также вводить затенённые переменные, как это можно сделать в `match` ветках: строка `if let Ok(age) = age` вводит новую затенённую переменную `age`, которая содержит значение внутри варианта `Ok`. Это означает, что нам нужно поместить условие `if age > 30` внутри этого блок: мы не можем объединить эти два условия в `if let Ok(age) = age && age > 30`. Затенённый `age`, который мы хотим сравнить с 30, не является действительным, пока не начнётся новая область видимости с фигурной скобки.

The downside of using `if let` expressions is that the compiler doesn’t check for exhaustiveness, whereas with `match` expressions it does. If we omitted the last `else` block and therefore missed handling some cases, the compiler would not alert us to the possible logic bug.

### Условные циклы `while let`

Similar in construction to `if let`, the `while let` conditional loop allows a `while` loop to run for as long as a pattern continues to match. In Listing 18-2 we code a `while let` loop that uses a vector as a stack and prints the values in the vector in the opposite order in which they were pushed.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-02/src/main.rs:here}}
```

<span class="caption">Листинг 18-2: Использование цикла <code>while let</code> для печати значений до тех пор, пока <code>stack.pop()</code> возвращает <code>Some</code></span>

В этом примере выводится 3, 2, а затем 1. Метод `pop` извлекает последний элемент из вектора и возвращает `Some(value)`. Если вектор пуст, то `pop` возвращает `None`. Цикл `while` продолжает выполнение кода в своём блоке, пока `pop` возвращает `Some`. Когда `pop` возвращает `None`, цикл останавливается. Мы можем использовать `while let` для удаления каждого элемента из стека.

### Цикл `for`

In a `for` loop, the value that directly follows the keyword `for` is a pattern. For example, in `for x in y` the `x` is the pattern. Listing 18-3 demonstrates how to use a pattern in a `for` loop to destructure, or break apart, a tuple as part of the `for` loop.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-03/src/main.rs:here}}
```

<span class="caption">Листинг 18-3: Использование шаблона в цикле <code>for</code> для деструктурирования кортежа</span>

Код в листинге 18-3 выведет следующее:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-03/output.txt}}
```

We adapt an iterator using the `enumerate` method so it produces a value and the index for that value, placed into a tuple. The first value produced is the tuple `(0, 'a')`. When this value is matched to the pattern `(index, value)`, `index` will be `0` and `value` will be `'a'`, printing the first line of the output.

### Оператор `let`

До этой главы мы подробно обсуждали только использование шаблонов с `match` и `if let`, но на самом деле, мы использовали шаблоны и в других местах, в том числе в операторах `let`. Например, рассмотрим следующее простое назначение переменной с помощью `let`:

```rust
let x = 5;
```

Every time you've used a `let` statement like this you've been using patterns, although you might not have realized it! More formally, a `let` statement looks like this:

```text
let PATTERN = EXPRESSION;
```

В выражениях типа `let x = 5;` с именем переменной в слоте `PATTERN`, имя переменной является просто отдельной, простой формой шаблона. Rust сравнивает выражение с шаблоном и присваивает любые имена, которые он находит. Так что в примере `let x = 5;`, `x` - это шаблон, который означает "привязать то, что соответствует здесь, переменной `x`". Поскольку имя `x` является полностью шаблоном, этот шаблон фактически означает "привязать все к переменной `x` независимо от значения".

Чтобы более чётко увидеть аспект сопоставления с шаблоном `let`, рассмотрим листинг 18-4, в котором используется шаблон с `let` для деструктурирования кортежа.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-04/src/main.rs:here}}
```

<span class="caption">Листинг 18-4. Использование шаблона для деструктуризации кортежа и создания трёх переменных одновременно</span>

Здесь мы сопоставляем кортеж с шаблоном. Rust сравнивает значение `(1, 2, 3)` с шаблоном `(x, y, z)` и видит, что значение соответствует шаблону, поэтому Rust связывает `1` с `x`, `2` с `y` и `3` с `z`. Вы можете думать об этом шаблоне кортежа как о вложении в него трёх отдельных шаблонов переменных.

Если количество элементов в шаблоне не совпадает с количеством элементов в кортеже, то весь тип не будет совпадать и мы получим ошибку компилятора. Например, в листинге 18-5 показана попытка деструктурировать кортеж с тремя элементами в две переменные, что не будет работать.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-05/src/main.rs:here}}
```

<span class="caption">Листинг 18-5: Неправильное построение шаблона, переменные не соответствуют количеству элементов в кортеже</span>

Попытка скомпилировать этот код приводит к ошибке:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-05/output.txt}}
```

To fix the error, we could ignore one or more of the values in the tuple using `_` or `..`, as you’ll see in the [“Ignoring Values in a Pattern”](ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern)<!-- ignore --> section. If the problem is that we have too many variables in the pattern, the solution is to make the types match by removing variables so the number of variables equals the number of elements in the tuple.

### Параметры функции

Параметры функции также могут быть образцами. Код в листинге 18-6 объявляет функцию с именем `foo`, которая принимает один параметр с именем `x` типа `i32`, к настоящему времени это должно выглядеть знакомым.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-06/src/main.rs:here}}
```

<span class="caption">Листинг 18-6: Сигнатура функции использует образцы в параметрах</span>

`x` это часть шаблона! Как и в случае с `let`, мы можем сопоставить кортеж в аргументах функции с образцом. Листинг 18-7 разделяет значения в кортеже при его передачи в функцию.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-07/src/main.rs}}
```

<span class="caption">Листинг 18-7: Функция с параметрами, которая разрушает кортеж</span>

Этот код печатает `текущие координаты: (3, 5)`. Значения `&(3, 5)` соответствуют образцу `&(x, y)`, поэтому `x` - это значение `3`, а `y` - это значение `5`.

Добавляя к вышесказанному, мы можем использовать шаблоны в списках параметров замыкания таким же образом, как и в списках параметров функции, потому что, как обсуждалось в главе 13, замыкания похожи на функции.

На данный момент вы видели несколько способов использования шаблонов, но шаблоны работают не одинаково во всех местах, где их можно использовать. В некоторых местах шаблоны должны быть неопровержимыми; в других обстоятельствах они могут быть опровергнуты. Мы обсудим эти две концепции далее.
