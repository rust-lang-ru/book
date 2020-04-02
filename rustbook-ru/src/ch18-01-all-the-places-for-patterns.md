## Все места для использования шаблонов

Паттерны появляются в нескольких местах Rust, и вы часто их используете, даже не осознавая этого! В этом разделе обсуждаются все места, где использование шаблонов является корректным.

### Рукава `match`

Как мы уже обсуждали в главе 6, основным местом, где шаблоны используются - это
рукава выражений `match`. Формально, выражения `match` определятся как объединение
ключевого слова `match`, значения которое будет сравниваться и одно или несколько
рукавов, которые составляют шаблон и выражение, которое будет выполнено, если
значение будет соответствовать шаблону:

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Одно из требований к выражениям `match` состоит в том, что они должны быть *исчерпывающими* (exhaustive) в том смысле, что они должны учитывать все возможности для значения в выражении `match`. Один из способов убедиться, что вы рассмотрели каждую возможность - это иметь шаблон перехвата всех вариантов в последнем рукаве выражения: например, имя переменной, совпадающее с любым значением, никогда не может потерпеть неудачу и таким образом, охватывает каждый оставшийся случай.

Конкретный шаблон `_` будет соответствовать чему угодно, но он никогда не привязывается к переменной, поэтому он часто используется в последнем рукаве. Шаблон `_` может быть полезен, если вы, например, хотите игнорировать любое не указанное значение. Мы рассмотрим шаблон `_` более подробно в разделе ["Игнорирование значений в шаблоне](ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern)<comment></comment> позже в этой главе.

### Условные выражения `if let`

В главе 6 мы обсуждали, как использовать выражения `if let` как правило в качестве более короткого способа записи эквивалента `match`, которое обрабатывает только один случай. Дополнительно `if let` может иметь соответствующий `else`, содержащий код для выполнения, если шаблон выражения `if let` не совпадает.

Listing 18-1 shows that it’s also possible to mix and match `if let`, `else if`, and `else if let` expressions. Doing so gives us more flexibility than a
`match` expression in which we can express only one value to compare with the
patterns. Also, the conditions in a series of `if let`, `else if`, `else if let` arms aren’t required to relate to each other.

The code in Listing 18-1 shows a series of checks for several conditions that
decide what the background color should be. For this example, we’ve created
variables with hardcoded values that a real program might receive from user
input.

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

<span class="caption">Листинг 18-1: Смешанное использование <code>if let</code>, <code>else if</code>, <code>else if let</code> и <code>else</code></span>

If the user specifies a favorite color, that color is the background color. If
today is Tuesday, the background color is green. If the user specifies
their age as a string and we can parse it as a number successfully, the color
is either purple or orange depending on the value of the number. If none of
these conditions apply, the background color is blue.

This conditional structure lets us support complex requirements. With the
hardcoded values we have here, this example will print `Using purple as the background color`.

You can see that `if let` can also introduce shadowed variables in the same way
that `match` arms can: the line `if let Ok(age) = age` introduces a new
shadowed `age` variable that contains the value inside the `Ok` variant. This
means we need to place the `if age > 30` condition within that block: we can’t
combine these two conditions into `if let Ok(age) = age && age > 30`. The
shadowed `age` we want to compare to 30 isn’t valid until the new scope starts
with the curly bracket.

Недостатком использования `if let` выражений является то, что компилятор не проверяет полноту (exhaustiveness) всех вариантов, в то время как с помощью выражения `match` это происходит. Если мы пропустим последний блок `else` и поэтому пропустим обработку некоторых случаев, компилятор не предупредит нас о возможной логической ошибке.

### Условные циклы `while let`

Аналогично конструкции `if let`, конструкция условного цикла `while let` позволяет условию `while` цикла работать до тех пор, пока шаблон продолжает совпадать. Пример в листинге 18-2 демонстрирует цикл `while let`, который использует вектор в качестве стека и печатает значения вектора в обратном порядке, в котором они были помещены.

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

<span class="caption">Листинг 18-2: Использование цикла <code>while let</code> для печати значений до тех пор, пока <code>stack.pop()</code> возвращает <code>Some</code></span>

В этом примере выводится 3, 2, а затем 1. Метод `pop` извлекает последний элемент из вектора и возвращает `Some(value)`. Если вектор пуст, то `pop` возвращает `None`. Цикл `while` продолжает выполнение кода в своем блоке, пока `pop` возвращает `Some`. Когда `pop` возвращает `None`, цикл останавливается. Мы можем использовать `while let` для удаления каждого элемента из стека.

### Цикл `for`

В главе 3 мы упоминали, что цикл `for` является самой распространенной конструкцией цикла в коде Rust, но мы еще не обсуждали шаблон, который использует `for`. В цикле  `for`, шаблон - это значение, которое следует непосредственно за ключевым словом `for`, поэтому `for x in y` `x` является шаблоном.

В листинге 18-3 показано, как использовать шаблон в цикле `for` для деструктурирования или разбиения кортежа как части цикла `for` .

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

<span class="caption">Листинг 18-3: Использование шаблона в цикле <code>for</code> для деструктурирования кортежа</span>

Код в листинге 18-3 выведет следующее:

```text
a is at index 0
b is at index 1
c is at index 2
```

Мы используем метод `enumerate` чтобы адаптировать итератор для создания значения и индекса этого значения в итераторе, помещенном в кортеж. Первый вызов `enumerate` производит кортеж `(0, 'a')` . Когда это значение сопоставляется с шаблоном `(index, value)`, `index` будет равен `0` а `value` будет равно `'a'` и будет напечатана первая строка выходных данных.

### Оператор `let`

До этой главы мы подробно обсуждали только использование шаблонов с `match` и `if let`, но на самом деле, мы использовали шаблоны и в других местах, в том числе в операторах `let`. Например, рассмотрим следующее простое назначение переменной с помощью `let`:

```rust
let x = 5;
```

На протяжении всей этой книги мы использовали `let` сотни раз, хотя вы, возможно, не поняли, что вы использовали шаблоны! Более формально, выражение `let` выглядит так:

```text
let PATTERN = EXPRESSION;
```

В выражениях типа `let x = 5;` с именем переменной в слоте `PATTERN`, имя переменной является просто отдельной, простой формой шаблона. Rust сравнивает выражение с шаблоном и присваивает любые имена, которые он находит. Так что в примере `let x = 5;`, `x` - это шаблон, который означает "привязать то, что соответствует здесь, переменной `x`". Поскольку имя `x` является полностью шаблоном, этот шаблон фактически означает "привязать все к переменной `x` независимо от значения".

Чтобы более четко увидеть аспект сопоставления с шаблоном `let`, рассмотрим листинг 18-4, в котором используется шаблон с `let` для деструктурирования кортежа.

```rust
let (x, y, z) = (1, 2, 3);
```

<span class="caption">Листинг 18-4: Использование шаблона для деструктурирования кортежа и одновременного создания трех переменных</span>

Здесь мы сопоставляем кортеж с шаблоном. Rust сравнивает значение `(1, 2, 3)` с шаблоном `(x, y, z)` и видит, что значение соответствует шаблону, поэтому Rust
привязывает `1` к `x`,  `2` к `y` и `3` к `z`. Можно думать об этом шаблоне кортежа как о вложении трех отдельных переменных шаблона внутри него.

Если количество элементов в шаблоне не совпадает с количеством элементов в кортеже, то весь тип не будет совпадать и мы получим ошибку компилятора. Например, в листинге 18-5 показана попытка деструктурировать кортеж с тремя элементами в две переменные, что не будет работать.

```rust,ignore,does_not_compile
let (x, y) = (1, 2, 3);
```

<span class="caption">Листинг 18-5: Неправильное построение шаблона, переменные не соответствуют количеству элементов в кортеже</span>

Попытка скомпилировать этот код приводит к ошибке:

```text
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^ expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected type `({integer}, {integer}, {integer})`
             found type `(_, _)`
```

Если бы мы хотели игнорировать одно или несколько значений в кортеже, мы могли бы использовать `_` или `..`, как вы увидите это в разделе [“Игнорирование значений в Шаблоне”](ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern)<comment></comment>. Если проблема в том, что у нас слишком много переменных в шаблоне, решение состоит в том, чтобы сделать так что типы совпадают, удаляя некоторые переменные и уравнивая число переменных к числу элементов в кортеже.

### Параметры функции

Function parameters can also be patterns. The code in Listing 18-6, which
declares a function named `foo` that takes one parameter named `x` of type
`i32`, should by now look familiar.

```rust
fn foo(x: i32) {
    // code goes here
}
```

<span class="caption">Листинг 18-6: Сигнатура функции использует образцы в
параметрах</span>

`x` часть это шаблон! Как и в случае с `let`, мы можем сопоставить кортеж в аргументах функции с образцом. Листинг 18-7 разделяет значения в кортеже при его передачи в функцию.

<span class="filename">Filename: src/main.rs</span>

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

<span class="caption">Листинг 18-7: Функция с параметрами, которая разрушает кортеж</span>

This code prints `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` is the value `3` and `y` is the value `5`.

Мы также можем использовать шаблоны в списках параметров замыкания так же, как в списках параметров функции, потому что замыкания похожи на функции, как обсуждалось в главе 13.

На данный момент вы видели несколько способов использования шаблонов, но шаблоны работают не одинаково во всех местах, где их можно использовать. В некоторых местах шаблоны должны быть неопровержимыми; в других обстоятельствах они могут быть опровергнуты. Мы обсудим эти две концепции далее.
