## Обрабатываемые ошибки и `Result`

Множество ошибок не являются настолько критичными, чтобы останавливать выполнение
программы. Весьма часто необходима просто правильная их обработка. К примеру, при
открытии файла может произойти ошибка из-за отсутствия файла. Решения могут быть
разные: от игнорирования до создания нового файла.

Надеюсь, что вы ещё помните содержание главы 2, где мы рассматривали перечисление
`Result`. Оно имеет два значения `Ok` и `Err`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` и `E` параметры перечисления. `T`  - это тип, которые будет возвращён, при
успехе, а `E` при ошибке.

Давайте вызовем функцию, которая возвращает значение `Result`, потому что эта функция
может потерпеть неудачу. В листинге 9-2 мы пытаемся открыть файл.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<span class="caption">Listing 9-2: Открытие файла</span>

Интересно, как узнать, какой тип возвращает метод `File::open`. Это просто. Надо
поставить тип  данных, который точно не подойдет и увидим тип данных в описании
ошибки:

```rust,ignore,does_not_compile
let f: u32 = File::open("hello.txt");
```

Информационное сообещение:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  = note:    found type `std::result::Result<std::fs::File, std::io::Error>`
```

Всё, я думаю, ясно из описания.

Для обработки исключительной ситуации необходимо добавить следующий код:

In the case where `File::open` succeeds, the value in the variable `f` will be
an instance of `Ok` that contains a file handle. In the case where it fails,
the value in `f` will be an instance of `Err` that contains more information
about the kind of error that happened.

We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

<span class="caption">Listing 9-3: Использование выражения <code>match</code> для обработки
<code>Result</code></span>

Обратите внимание, что перечисление `Result`, также как `Option`, входит в состав
экспорта по умолчанию.

Здесь мы сообщаем, что значение `Ok` содержит значение `file` типа `File`.
Другое значение может хранить значение типа `Err`. В этом примере мы используем
вызов макроса `panic!`. Если нет файла с именем *hello.txt*, будет выполнен этот код.
Следовательно, будет выведено следующее сообщение:

The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we’ve chosen to call the `panic!` macro. If
there’s no file named *hello.txt* in our current directory and we run this
code, we’ll see the following output from the `panic!` macro:

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

As usual, this output tells us exactly what has gone wrong.

### Обработка различных ошибок

Пример создание нового файла при отсутствии запрашиваемого файла:

<span class="filename">Filename: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
    print!("{:#?}",f);
}
```

<span class="caption">Listing 9-4: Обработка различных ошибок несколькими способами</span>

The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value. The enum
`io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we’re trying to open doesn’t exist yet. So we match on `f`, but we
also have an inner match on `error.kind()`.

The condition we want to check in the inner match is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need a second arm in the inner `match` expression. When the
file can’t be created, a different error message is printed. The second arm of
the outer `match` stays the same, so the program panics on any error besides
the missing file error.

That’s a lot of `match`! The `match` expression is very useful but also very
much a primitive. In Chapter 13, you’ll learn about closures; the `Result<T, E>` type has many methods that accept a closure and are implemented using
`match` expressions. Using those methods will make your code more concise. A
more seasoned Rustacean might write this code instead of Listing 9-5:

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

Although this code has the same behavior as Listing 9-5, it doesn’t contain any
`match` expressions and is cleaner to read. Come back to this example after
you’ve read Chapter 13, and look up the `unwrap_or_else` method in the standard
library documentation. Many more of these methods can clean up huge nested
`match` expressions when you’re dealing with errors.

### Сокращенные макросы обработки ошибок `unwrap` и `expect`

Метод `unwrap` - это оболочка выражения `match`, которая возвращает `Ok` или `Err`.

<span class="filename">Если мы выполним код без наличия файла <em>hello.txt</em>, будет выведена ошибка:</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    print!("{:#?}", f);
}
```

Есть ещё один метод похожий на `unwrap`. Используя `expect` вместо `unwrap` и
предоставляющий хорошие информативные описания ошибок::

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
/stable-dist-rustc/build/src/libcore/result.rs:868
```

Another method, `expect`, which is similar to `unwrap`, lets us also choose the
`panic!` error message. Using `expect` instead of `unwrap` and providing good
error messages can convey your intent and make tracking down the source of a
panic easier. The syntax of `expect` looks like this:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    print!("{:?}", f);
}
```

Мы используем `expect` таким же образом, каким и `unwrap`: возвращаем ссылку на файл или
вызов макроса `panic!`.

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Because this error message starts with the text we specified, `Failed to open hello.txt`, it will be easier to find where in the code this error message is
coming from. If we use `unwrap` in multiple places, it can take more time to
figure out exactly which `unwrap` is causing the panic because all `unwrap`
calls that panic print the same message.

### Генерировании ошибок

Когда вы пишите функцию, в результате работы которой может произойти непредвиденная
ошибка, вместо того, чтобы обрабатывать эту ошибку вы можете создать подробное
описание этой и передать ошибку по цепочке на верхний уровень обработки кода.

Например, код программы 9-5 читает имя пользователя из файла. Если файл не существует
или не может быть прочтён, то функция возвращает эту ошибку в код, который вызвал
эту функцию:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

<span class="caption">Listing 9-6: A function that returns errors to the
calling code using <code>match</code></span>

This function can be written in a much shorter way, but we’re going to start by
doing a lot of it manually in order to explore error handling; at the end,
we’ll show the shorter way. Let’s look at the return type of the function first:
`Result<String, io::Error>`. This means the function is returning a value of
the type `Result<T, E>` where the generic parameter `T` has been filled in
with the concrete type `String` and the generic type `E` has been filled in
with the concrete type `io::Error`. If this function succeeds without any
problems, the code that calls this function will receive an `Ok` value that
holds a `String`—the username that this function read from the file. If this
function encounters any problems, the code that calls this function will
receive an `Err` value that holds an instance of `io::Error` that contains
more information about what the problems were. We chose `io::Error` as the
return type of this function because that happens to be the type of the error
value returned from both of the operations we’re calling in this function’s
body that might fail: the `File::open` function and the `read_to_string`
method.

The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value returned with a `match` similar to the `match` in
Listing 9-4, only instead of calling `panic!` in the `Err` case, we return
early from this function and pass the error value from `File::open` back to the
calling code as this function’s error value. If `File::open` succeeds, we store
the file handle in the variable `f` and continue.

Тело этой функции начинается с вызову функции `File::open`. Далее, мы получаем результат
анализа результата чтения файла функцией `match`. Если функция `File::open` сработала
успешно, мы сохраняет ссылку на файл в переменную `f` и программа продолжает свою
работу.

Далее, мы создаём строковую переменную `s` и вызываем метод файла `read_to_string`,
который читает содержание файла, как строковые данные, в переменную `s`. Результатом
работы этой фунции будет значение перечисления `Result`: `Ok` или `io::Error`.

Этого же результата можно достичь с помощью сокращенного написания (с помощью использования
символа `?`).

#### Сокращение описание `match`  `?`

Код программы 9-6 показывает реализацию функции `read_username_from_file`, функционал
которой аналогичен коду программы 9-5, но имеет сокращённое описание:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<span class="caption">Код программы 9-6: Пример функции, которая возвращает ошибку,
используя символ <code>?</code></span>

Благодаря использованию символа `?` сокращается запись кода (код, написанный в
предыдущем примере создаётся компилятором самостоятельно).

There is a difference between what the `match` expression from Listing 9-6 and
the `?` operator do: error values that have the `?` operator called on them go
through the `from` function, defined in the `From` trait in the standard
library, which is used to convert errors from one type into another. When the
`?` operator calls the `from` function, the error type received is converted
into the error type defined in the return type of the current function. This is
useful when a function returns one error type to represent all the ways a
function might fail, even if parts might fail for many different reasons. As
long as each error type implements the `from` function to define how to convert
itself to the returned error type, the `?` operator takes care of the
conversion automatically.

В коде примера 9-6 в первой строке функция `File::open` возвращает содержимое значения
перечисления `Ok` в переменную `f`. Если же в при работе этой функции происходит
ошибка, будет возвращен экземпляр структуры `Err`. Те же самые действия произойдут
при чтении текстовых данных из файла с помощью функции `read_to_string`.

Использование сокращенных конструкций позволят уменьшить количество строк кода и
место потенциальных ошибок. Написанный в предыдущем примере сокращенный код можно
сделать ещё меньше с помощью сокращения промежуточных переменных и конвейерного вызова
методов:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

<span class="caption">Listing 9-8: Chaining method calls after the <code>?</code>
operator</span>

Мы перенесли создание экземпляра структуры `String` в начало функции. Вместо того,
чтобы создавать переменную `f` мы последовательно вызываем методы экземпляров
выходные данных.

Speaking of different ways to write this function, Listing 9-9 shows that
there’s a way to make this even shorter.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

<span class="caption">Listing 9-9: Using <code>fs::read_to_string</code> instead of
opening and then reading the file</span>

Reading a file into a string is a fairly common operation, so Rust provides the
convenient `fs::read_to_string` function that opens the file, creates a new
`String`, reads the contents of the file, puts the contents into that `String`,
and returns it. Of course, using `fs::read_to_string` doesn’t give us the
opportunity to explain all the error handling, so we did it the longer way
first.

#### Оператор `?` можно использовать для функций возвращающих `Result`

Сокращенную запись с помощью символа `?` можно использовать в функциях, которые возвращают значение перечисления `Result`. Соответственно, если функция не возвращает значение перечисления `Result`, а в коде написано обратное - компилятор сгенерирует
ошибку. Пример:

Let’s look at what happens if we use the `?` operator in the `main` function,
which you’ll recall has a return type of `()`:

```rust,ignore,does_not_compile
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

Описание ошибки:

```text
error[E0277]: the `?` operator can only be used in a function that returns
`Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a
  function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
```

Данная ошибка указывает, что можно использовать оператор `?` в функции, возвращающей `Result<T, E>`. Когда вы пишите код в функции, которая не возвращает `Result<T, E>` и хотите использовать оператор `?`, при вызове других функций, которые возвращают `Result<T, E>`, то у вас есть два способ исправить проблему. Изменить возвращаемый тип вашей функции на `Result<T, E>`, если ничего не ограничивает это сделать. Другая техника это использование `match` или одного из методов типа  `Result<T, E>` для обработки `Result<T, E>` любым подходящим способом.

The `main` function is special, and there are restrictions on what its return
type must be. One valid return type for main is `()`, and conveniently, another
valid return type is `Result<T, E>`, as shown here:

```rust,ignore
error[E0308]: mismatched types
 -->
  |
3 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum
`std::result::Result`
  |
  = note: expected type `()`
  = note:    found type `std::result::Result<_, _>`
```

В описании ошибки сообщается, что функция `main` должна возвращать кортеж, а вместо
этого - функция возвращает `Result`.

В следующем разделе будет рассказано об особенностях вызова макроса `panic!`, приведены
рекомендации при выборе конструкции для отслеживания ошибок.
