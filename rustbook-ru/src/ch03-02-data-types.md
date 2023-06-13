## Типы данных

Каждое значение в Rust имеет определённый *тип данных*, сообщающий Rust, какие данные используются и как с ними работать. Мы рассмотрим два подмножества типов данных: скалярные и сложные.

Не забывайте, что Rust является *статически типизированным* (statically typed) языком. Это означает, что он должен знать типы всех переменных во время компиляции. Обычно компилятор может предположить, какой тип используется (вывести его), основываясь на значении и на том, как мы с ним работаем. В случаях, когда может быть выведено несколько типов, необходимо добавлять аннотацию типа вручную. Например, когда мы конвертировали `String` в число с помощью вызова `parse` в разделе [«Сравнение предположения с загаданным номером»](ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) главы 2, мы должны добавить такую аннотацию:<!-- ignore -->

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the `: u32` type annotation shown in the preceding code, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

В будущем вы увидите различные аннотации для разных типов данных.

### Скалярные типы данных

*Скалярный* тип представляет единственное значение. В Rust есть четыре скалярных типа: целые и вещественные числа, логический тип и символы. Вы можете узнать эти типы по другим языкам программирования. Посмотрим, как они работают в Rust.

#### Целочисленные типы

An *integer* is a number without a fractional component. We used one integer type in Chapter 2, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with `i` instead of `u`) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

<span class="caption">Таблица 3-1: целочисленные типы в Rust</span>

Длина | Со знаком | Без знака
--- | --- | ---
8 бит | `i8` | `u8`
16-bit | `i16` | `u16`
32 бит | `i32` | `u32`
64 бит | `i64` | `u64`
128-bit | `i128` | `u128`
arch | `isize` | `usize`

Каждый вариант может быть как со знаком, так и без знака и имеет явный размер. Такая характеристика типа как *знаковый* и *беззнаковый* определяет возможность числа быть отрицательным. Другими словами, должно ли число иметь знак (знаковое) или оно всегда будет только положительным и, следовательно, может быть представлено без знака (беззнаковое). Это похоже на написание чисел на бумаге: когда знак имеет значение, число отображается со знаком плюс или со знаком минус; однако, когда можно с уверенностью предположить, что число положительное, оно отображается без знака. Числа со знаком хранятся с использованием [дополнительного кода](https://en.wikipedia.org/wiki/Two%27s_complement)<!-- ignore -->.

Каждый вариант со знаком может хранить числа от -(2 <sup>n - 1</sup> ) до 2 <sup>n - 1</sup> - 1 включительно, где *n* — количество битов, которые использует этот вариант. Таким образом, `i8` может хранить числа от -(2 <sup>7</sup> ) до 2 <sup>7</sup> - 1, что равно значениям от -128 до 127. Варианты без знака могут хранить числа от 0 до 2 <sup>n</sup> - 1, поэтому `u8` может хранить числа от 0 до 2 <sup>8</sup> - 1, что равно значениям от 0 до 255.

Кроме того, `isize` и `usize` зависят от архитектуры компьютера, на котором запускается ваша программа, что отражено в таблице как «arch»: 64-битный, если вы используете 64-битную архитектуру, и 32-битный, если вы на 32-битной архитектуре.

Целочисленные литералы можно записывать в любой из форм, показанных в таблице 3-2. Обратите внимание, что числовые литералы, которые могут быть несколькими числовыми типами, позволяют использовать суффикс типа, например `57u8`, для обозначения типа. Числовые литералы также могут использовать `_` в качестве визуального разделителя, чтобы число было легче читать, например `1_000`, которое будет иметь то же значение, как если бы вы указали `1000`.

<span class="caption">Таблица 3-2: Целочисленные литералы в Rust</span>

Number literals | Example
--- | ---
Decimal | `98_222`
Hex | `0xff`
Octal | `0o77`
Двоичный | `0b1111_0000`
Байт (только `u8`) | `b'A'`

Как же узнать, какой тип целого числа использовать? Если вы не уверены, значения по умолчанию в Rust, как правило, подходят для начала: целочисленные типы по умолчанию `i32`. Основной случай, в котором вы должны использовать `isize` или `usize`, — это индексация какой-либо коллекции.

>  ##### Integer Overflow
>  Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, *integer overflow* will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to *panic* at runtime if this behavior occurs. Rust uses the term *panicking* when a program exits with an error; we’ll discuss panics in more depth in the [“Unrecoverable Errors with `panic!`”](ch09-01-unrecoverable-errors-with-panic.html)<!-- ignore --> section in Chapter 9.
>  When you’re compiling in release mode with the `--release` flag, Rust does *not* include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs *two’s complement wrapping*. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.
>  To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
>  - Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
> - Return the `None` value if there is overflow with the `checked_*` methods.
> - Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
> - Saturate at the value’s minimum or maximum values with the `saturating_*` methods.
>

#### Числа с плавающей запятой

Rust also has two primitive types for *floating-point numbers*, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs, it’s roughly the same speed as `f32` but is capable of more precision. All floating-point types are signed.

Вот пример, демонстрирующий числа с плавающей запятой в действии:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Числа с плавающей точкой представлены согласно стандарту IEEE-754. Тип `f32` является числом с плавающей точкой одинарной точности, а `f64` имеет двойную точность.

#### Числовые операции

Rust supports the basic mathematical operations you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer. The following code shows how you’d use each numeric operation in a `let` statement:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Каждое из этих выражений использует математические операции и вычисляет значение, которое затем присваивается переменной. [Приложение Б](appendix-02-operators.html)<!-- ignore --> содержит список всех операторов, имеющихся в Rust.

#### Логический тип данных

Как и в большинстве языков программирования, логический тип в Rust может иметь два значения: `true` и `false` и занимает в памяти один байт. Логический тип в Rust аннотируется при помощи `bool`. Например:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Основной способ использования значений логического типа — условные конструкции, такие как выражение `if`. Мы расскажем про работу выражения `if` в разделе [«Условные конструкции»](ch03-05-control-flow.html#control-flow)<!-- ignore -->.

#### Символьный тип данных

Rust’s `char` type is the language’s most primitive alphabetic type. Here are some examples of declaring `char` values:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Обратите внимание, что мы указываем литералы `char` в одинарных кавычках, в отличие от строковых литералов, которые используют двойные кавычки. Тип `char` в Rust имеет размер четыре байта и представляет собой скалярное значение Unicode. Это значит, что он может представлять гораздо больше, чем просто ASCII. Буквы с ударением; китайские, японские и корейские иероглифы; эмодзи и пробелы нулевой ширины являются допустимыми значениями `char` в Rust. Скалярные значения Unicode находятся в диапазоне от `U+0000` до `U+D7FF` и от `U+E000` до `U+10FFFF` включительно. Однако «символ» на самом деле не является концепцией в Unicode, поэтому интуитивно может не совпадать с тем, что такое `char` в Rust. Мы подробно обсудим эту тему в разделе [«Сохранение текста в кодировке UTF-8 со строками»](ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)<!-- ignore --> в главе 8.

### Сложные типы данных

*Сложные типы* могут группировать несколько значений в один тип. В Rust есть два примитивных сложных (комбинированных) типа: кортежи и массивы.

#### Кортежи

Кортеж является общим способом совместной группировки нескольких значений различного типа в единый комбинированный тип. Кортежи имеют фиксированную длину: после объявления они не могут расти или уменьшаться в размере.

Кортеж создаётся при помощи записи списка значений, перечисленных через запятую внутри круглых скобок. Каждая позиция в кортеже имеет тип. Типы различных значений в кортеже могут не быть одинаковыми. В примере мы добавили необязательные аннотации типов:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

The variable `tup` binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

This program first creates a tuple and binds it to the variable `tup`. It then uses a pattern with `let` to take `tup` and turn it into three separate variables, `x`, `y`, and `z`. This is called *destructuring* because it breaks the single tuple into three parts. Finally, the program prints the value of `y`, which is `6.4`.

Мы также можем напрямую обращаться к элементу кортежа, используя точку (`.`), за которой следует индекс значения, к которому мы хотим получить доступ. Например:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Эта программа создаёт кортеж `x`, а затем обращается к каждому элементу кортежа, используя соответствующие индексы. Как и в большинстве языков программирования, первый индекс в кортеже равен 0.

Кортеж без каких-либо значений имеет специальное имя *unit*. Это значение и соответствующий ему тип записываются как `()` и представляют собой пустое значение или пустой возвращаемый тип. Выражения неявно возвращают unit, если они не возвращают никакого другого значения.

#### Массивы

Другой способ получить набор из нескольких значений — это *массив*. В отличие от кортежа, каждый элемент массива должен иметь один и тот же тип. В отличие от массивов в некоторых других языках, массивы в Rust имеют фиксированную длину.

Мы записываем значения в массиве в виде списка, разделённого запятыми, внутри квадратных скобок:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Массивы полезны, когда вы хотите, чтобы ваши данные размещались в стеке, а не в куче (мы более подробно обсудим стек и кучу в [главе 4](ch04-01-what-is-ownership.html#the-stack-and-the-heap)<!-- ignore -->), или когда вы хотите, чтобы у вас всегда было фиксированное количество элементов. Однако массив не такой гибкий, как векторный тип. Вектор — это аналогичный тип коллекции, предоставляемый стандартной библиотекой, размер которого *может* увеличиваться или уменьшаться. Если вы не уверены, использовать массив или вектор, скорее всего, вам следует использовать вектор. [Глава 8](ch08-01-vectors.html)<!-- ignore --> раскрывает векторы более подробно.

Однако массивы более полезны, когда вы знаете, что количество элементов не нужно будет изменять. Например, если бы вы использовали названия месяцев в программе, вы, вероятно, использовали бы массив, а не вектор, потому что вы знаете, что он всегда будет содержать 12 элементов:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Тип массива записывается следующим образом: в квадратных скобках обозначается тип элементов массива, а затем, через точку с запятой, количество элементов. Например:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Здесь `i32` является типом каждого элемента массива. После точки с запятой указано число `5`, показывающее, что массив содержит 5 элементов.

Вы также можете инициализировать массив, содержащий одно и то же значение для каждого элемента, указав это значение вместо типа. Следом за этим так же следует точка с запятой, а затем — длина массива в квадратных скобках, как показано здесь:

```rust
let a = [3; 5];
```

Массив в переменной `a` будет включать `5` элементов, значение которых будет равно `3`. Данная запись аналогична коду `let a = [3, 3, 3, 3, 3];`, но является более краткой.

##### Доступ к элементам массива

Массив — это единый фрагмент памяти известного фиксированного размера, который может быть размещён в стеке. Вы можете получить доступ к элементам массива с помощью индексации, например:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

In this example, the variable named `first` will get the value `1` because that is the value at index `[0]` in the array. The variable named `second` will get the value `2` from index `[1]` in the array.

##### Некорректный доступ к элементу массива

Давайте посмотрим, что произойдёт, если вы попытаетесь получить доступ к элементу массива, находящемуся за его пределами. Допустим, вы запускаете код, похожий на игру в угадывание из главы 2, чтобы получить индекс массива от пользователя:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Этот код успешно компилируется. Если вы запустите его, используя команду `cargo run`, и введёте 0, 1, 2, 3 или 4, программа выведет соответствующее значение по этому индексу в массиве. Если вместо этого вы введёте число, указывающее на индекс за пределами массива, например 10, вы увидите следующий вывод:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Программа привела к ошибке *во время выполнения* в момент использования недопустимого значения в операции индексирования. Программа завершилась с сообщением об ошибке и не выполнила `println!`. Когда вы пытаетесь получить доступ к элементу с помощью индексации, Rust сам проверит, что указанный вами индекс меньше длины массива. Если индекс больше или равен длине, Rust запаникует. Но в данном случае эта проверка должна выполняться и во время работы программы, потому что компилятор не может знать, какое значение введёт пользователь, когда код будет запущен.

Это пример принципов безопасности памяти Rust в действии. Во многих низкоуровневых языках такая проверка не выполняется, и когда вы указываете неправильный индекс, доступ к памяти может быть некорректным. Rust защищает вас от такого рода ошибок, немедленно закрываясь вместо того, чтобы разрешать доступ к памяти и продолжать работу. В главе 9 подробнее обсуждается обработка ошибок в Rust и то, как вы можете написать читаемый, безопасный код, который не вызывает панику и не разрешает некорректный доступ к памяти.
