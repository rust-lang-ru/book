## Типы данных

Every value in Rust is of a certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

Не забывайте, что Rust является *статически типизированным* (statically typed) языком. Это означает, что он должен знать типы всех переменных во время компиляции. Обычно компилятор может вывести (*infer*) какой тип мы хотим использовать, основываясь на значении и на том, как мы с ним работаем. В случаях, когда может быть выведено несколько типов, необходимо вручную  добавлять аннотацию типа. Например, когда мы конвертировали `String` в число с помощью вызова `parse` в разделе ["Сравнение предположения с загаданным номером"](ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) Главы 2, мы должны добавить такую аннотацию:<!--  -->

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the `: u32` type annotation above, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

В будущем вы увидите различные аннотации для разных типов данных.

### Скалярные типы данных

*Скалярный* тип представляет единственное значение. В Rust есть четыре скалярных типа: целые и вещественные числа, логический тип и символы. Вы можете узнать эти типы по другим языкам программирования. Посмотрим на то, как они работают в Rust.

#### Integer Types

An *integer* is a number without a fractional component. We used one integer type in Chapter 2, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with `i`, instead of `u`) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

<span class="caption">Table 3-1: Integer Types in Rust</span>

Length | Signed | Unsigned
--- | --- | ---
8-bit | `i8` | `u8`
16-bit | `i16` | `u16`
32-bit | `i32` | `u32`
64-bit | `i64` | `u64`
128-bit | `i128` | `u128`
arch | `isize` | `usize`

Each variant can be either signed or unsigned and has an explicit size. *Signed* and *unsigned* refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using [two’s complement](https://en.wikipedia.org/wiki/Two%27s_complement)<!-- ignore --> representation.

Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n - 1</sup> - 1 inclusive, where *n* is the number of bits that variant uses. So an `i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1, so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.

Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as `57u8`, to designate the type. Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`, which will have the same value as if you had specified `1000`.

<span class="caption">Table 3-2: Integer Literals in Rust</span>

Number literals | Example
--- | ---
Decimal | `98_222`
Hex | `0xff`
Octal | `0o77`
Binary | `0b1111_0000`
Byte (`u8` only) | `b'A'`

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good places to start: integer types default to `i32`. The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection.

> ##### Integer Overflow
>
> Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, *integer overflow* will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to *panic* at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the [“Unrecoverable Errors with `panic!`”](ch09-01-unrecoverable-errors-with-panic.html)<!-- ignore --> section in Chapter 9.
>
> When you’re compiling in release mode with the `--release` flag, Rust does *not* include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs *two’s complement wrapping*. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.
>
> To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
>
> - Обернуть все режимы с помощью `wrapping_*` методов, например `wrapping_add`
> - Вернуть значение `None` в случае переполнения при помощи методов `checked_*`
> - Вернуть значение и логическое значение, указывающее, было ли переполнение с помощью методов `overflowing_*`
> - Saturate at the value’s minimum or maximum values with `saturating_*` methods

#### Числа с плавающей запятой

Rust also has two primitive types for *floating-point numbers*, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs it’s roughly the same speed as `f32` but is capable of more precision. All floating-point types are signed.

Here’s an example that shows floating-point numbers in action:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Числа с плавающей точкой представлены согласно стандарту IEEE-754. Тип `f32` является числом с плавающей точкой одинарной точности, а `f64` имеет двойную точность.

#### Числовые операции

Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder. Integer division rounds down to the nearest integer. The following code shows how you’d use each numeric operation in a `let` statement:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Каждое из этих выражений использует математические операции и вычисляет значение, которое затем присваивается переменной. ["Приложение Б"](appendix-02-operators.html)<!--  --> содержит список всех операторов, имеющихся в Rust.

#### Логический тип данных

Как и в большинстве языков программирования, логический тип в Rust может иметь два значения: `true` и `false`, и занимает в памяти один байт. Логический тип в Rust аннотируется при помощи `bool`. Например:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Основной способ использования значений логического типа - это условные конструкции, такие как выражение `if`. Мы расскажем про работу выражения `if` в разделе ["Условные конструкции"](ch03-05-control-flow.html#control-flow)<!--  -->.

#### The Character Type

Rust’s `char` type is the language’s most primitive alphabetic type. Here’s some examples of declaring `char` values:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Note that we specify `char` literals with single quotes, as opposed to string literals, which use double quotes. Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a `char` is in Rust. We’ll discuss this topic in detail in [“Storing UTF-8 Encoded Text with Strings”](ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)<!-- ignore --> in Chapter 8.

### Compound Types

*Сложные типы* могут группировать несколько значений в один тип. В Rust есть два примитивных сложных (комбинированных) типа: кортежи и массивы.

#### The Tuple Type

Кортеж является общим способом совместной группировки нескольких значений различного типа в единый комбинированный тип. Кортежи имеют фиксированную длину: после объявления они не могут расти или уменьшаться в размере.

Кортеж создаётся при помощи записи списка значений, перечисленных через запятую внутри круглых скобок. Каждая позиция в кортеже имеет тип. Типы различных значений в кортеже могут не быть одинаковыми. В примере мы добавили не обязательные аннотации типов:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Переменной с именем `tup` привязывается весь кортеж, потому что кортеж является единым комбинированным элементом. Чтобы получить отдельные значения из кортежа, можно использовать сопоставление с образцом для деструктурирования значений кортежа, как в примере:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Программа создаёт кортеж, привязывает его к переменной `tup`. Затем в `let` используется шаблон для превращения `tup` в три отдельных переменные: `x`, `y` и `z`. Такого рода операция называется *деструктуризацией* (destructuring), потому что она разрушает один кортеж на три части. В конце программа печатает значение `y`, которое равно `6.4`.

We can also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

This program creates the tuple `x` and then accesses each element of the tuple using their respective indices. As with most programming languages, the first index in a tuple is 0.

The tuple without any values has a special name, *unit*. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

#### The Array Type

Another way to have a collection of multiple values is with an *array*. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

We write the values in an array as a comma-separated list inside square brackets:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in [Chapter 4](ch04-01-what-is-ownership.html#the-stack-and-the-heap)<!-- ignore -->) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that *is* allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector. [Chapter 8](ch08-01-vectors.html)<!-- ignore --> discusses vectors in more detail.

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Здесь, `i32` является типом каждого элемента массива. После точки с запятой указано число `5` показывающее, что массив содержит 5 элементов.

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

Массив в переменной `a` будет включать `5` элементов, значение которых будет равно `3`. Данная запись аналогична коду `let a = [3, 3, 3, 3, 3];`, но является более краткой.

##### Доступ к элементам массива

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

In this example, the variable named `first` will get the value `1`, because that is the value at index `[0]` in the array. The variable named `second` will get the value `2` from index `[1]` in the array.

##### Некорректный доступ к элементу массива

Let’s see what happens if you try to access an element of an array that is past the end of the array. Say you run this code, similar to the guessing game in Chapter 2, to get an array index from the user:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

This code compiles successfully. If you run this code using `cargo run` and enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at that index in the array. If you instead enter a number past the end of the array, such as 10, you’ll see output like this:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The program resulted in a *runtime* error at the point of using an invalid value in the indexing operation. The program exited with an error message and didn’t execute the final `println!` statement. When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime, especially in this case, because the compiler can’t possibly know what value a user will enter when they run the code later.

This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. Chapter 9 discusses more of Rust’s error handling and how you can write readable, safe code that neither panics nor allows invalid memory access.
