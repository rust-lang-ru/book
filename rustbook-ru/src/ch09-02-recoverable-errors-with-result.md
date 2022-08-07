## Исправимые ошибки с `Result`

Многие ошибки являются не настолько критичными, чтобы останавливать выполнение программы. Иногда, когда в функции происходит сбой, необходима просто правильная интерпретация и обработка ошибки. К примеру, при попытке открыть файл может произойти ошибка из-за отсутствия файла. Вы, возможно, захотите исправить ситуацию и создать новый файл вместо остановки программы.

Вспомните раздел ["Обработка потенциального сбоя с помощью типа `Result`"]<!-- ignore --> главы 2: мы использовали там перечисление `Result`, имеющее два варианта, `Ok` и `Err` для обработки сбоев. Само перечисление определено следующим образом:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Типы `T` и `E` являются параметрами обобщённого типа: мы обсудим обобщённые типы более подробно в Главе 10. Все что вам нужно знать прямо сейчас - это то, что `T` представляет тип значения, которое будет возвращено в случае успеха внутри варианта `Ok`, а `E` представляет тип ошибки, которая будет возвращена при сбое внутри варианта `Err`. Так как тип `Result` имеет эти обобщённые параметры (generic type parameters), мы можем использовать тип `Result` и функции, которые определены для него, в разных ситуациях, когда тип успешного значение и значения ошибки, которые мы хотим вернуть, отличаются.

Давайте вызовем функцию, которая возвращает значение `Result`, потому что может потерпеть неудачу. В листинге 9-3 мы пытаемся открыть файл.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Листинг 9-3: Открытие файла</span>

The return type of `File::open` is a `Result<T, E>`. The generic parameter `T` has been filled in by the implementation of `File::open` with the type of the success value, `std::fs::File`, which is a file handle. The type of `E` used in the error value is `std::io::Error`. This return type means the call to `File::open` might succeed and return a file handle that we can read from or write to. The function call also might fail: for example, the file might not exist, or we might not have permission to access the file. The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. This information is exactly what the `Result` enum conveys.

In the case where `File::open` succeeds, the value in the variable `greeting_file_result` will be an instance of `Ok` that contains a file handle. In the case where it fails, the value in `greeting_file_result` will be an instance of `Err` that contains more information about the kind of error that happened.

Необходимо дописать в код листинга 9-3 выполнение разных действий в зависимости от значения, которое вернёт вызов `File::open`. Листинг 9-4 показывает один из способов обработки `Result` - пользуясь базовым инструментом языка, таким как выражение `match`, рассмотренным в Главе 6.

<span class="filename">Файл: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Листинг 9-4: Использование выражения <code>match</code> для обработки возвращаемых вариантов типа <code>Result</code></span>

Обратите внимание, что также как перечисление `Option`, перечисление `Result` и его варианты, входят в область видимости благодаря авто-импорту (prelude), поэтому не нужно указывать `Result::` перед использованием вариантов `Ok` и `Err` в ветках выражения `match`.

When the result is `Ok`, this code will return the inner `file` value out of the `Ok` variant, and we then assign that file handle value to the variable `greeting_file`. After the `match`, we can use the file handle for reading or writing.

Другая ветвь `match` обрабатывает случай, где мы получаем значение `Err` после вызова `File::open`. В этом примере мы решили вызвать макрос `panic!`. Если в нашей текущей директории нет файла с именем *hello.txt* и мы выполним этот код, то мы увидим следующее сообщение от макроса `panic!`:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Как обычно, данное сообщение точно говорит, что пошло не так.

### Обработка различных ошибок с помощью match

Код в листинге 9-4 будет вызывать `panic!` независимо от того, почему вызов `File::open` не удался. Однако мы хотим предпринять различные действия для разных причин сбоя. Если открытие `File::open` не удалось из-за отсутствия файла, мы хотим создать файл и вернуть его дескриптор. Если вызов `File::open` не удался по любой другой причине - например, потому что у нас не было прав на открытие файла, то все равно мы хотим вызвать `panic!` как у нас сделано в листинге 9-4. Для этого мы добавляем выражение внутреннего `match`, показанное в листинге 9-5.

<span class="filename">Файл: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Листинг 9-5: Обработка различных ошибок разными способами</span>

Типом значения возвращаемого функцией `File::open` внутри `Err` варианта является `io::Error`, структура из стандартной библиотеки. Данная структура имеет метод `kind`, который можно вызвать для получения значения `io::ErrorKind`. Перечисление `io::ErrorKind` из стандартной библиотеки имеет варианты, представляющие различные типы ошибок, которые могут появиться при выполнении операций в `io`. Вариант, который мы хотим использовать, это `ErrorKind::NotFound`, который даёт информацию, о том, что файл который мы пытаемся открыть ещё не существует. Итак, во второй строке мы вызываем сопоставление шаблона с переменной `greeting_file_result` и попадаем в ветку с обработкой ошибки, но также у нас есть внутренняя проверка для сопоставления `error.kind()` ошибки.

Условие, которое мы хотим проверить во внутреннем `match`, заключается в том, является ли значение, возвращаемое `error.kind()`, вариантом `NotFound` перечисления `ErrorKind`. Если это так, мы пытаемся создать файл с помощью функции `File::create`. Однако, поскольку вызов `File::create` тоже может завершиться ошибкой, нам нужна обработка ещё одной ошибки, теперь уже во внутреннем выражении `match`. Заметьте: если файл не может быть создан, выводится другое, специализированное сообщение об ошибке. Вторая же ветка внешнего `match` (который обрабатывает вызов `error.kind()`), остаётся той же самой - в итоге программа паникует при любой ошибке, кроме ошибки отсутствия файла.

> ### Альтернативы использованию `match` с `Result<T, E>`
>
> Как много `match`! Выражение `match` является очень полезным, но в то же время довольно примитивным. В главе 13 вы узнаете о замыканиях (closures), которые используются во многих методах типа `Result<T, E>`. Эти методы помогают быть более лаконичным, чем использование `match` при работе со значениями `Result<T, E>` в вашем коде.
>
> Например, вот другой способ написать ту же логику, что показана в Листинге 9-5, но с использованием замыканий и метода `unwrap_or_else`:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> Хотя этот код ведёт себя так же, как и код из листинга 9-5, он не содержит никаких выражений `match` и его легче читать. После прочтения главы 13 и поищите метод `unwrap_or_else` в документации по стандартной библиотеке. Множество других подобных методов могут очистить огромные вложенные выражения match, когда вы имеете дело с ошибками.

### Лаконичные способы обработки ошибок - `unwrap` и `expect`

Использование `match` работает достаточно хорошо, но может быть довольно многословным и не всегда хорошо передаёт смысл. Тип `Result<T, E>` имеет множество вспомогательных методов для выполнения различных, более специфических задач. Метод `unwrap` - это метод быстрого доступа к значениям, реализованный так же, как и выражение `match`, которое мы написали в Листинге 9-4. Если значение `Result` является вариантом `Ok`, `unwrap` возвращает значение внутри `Ok`. Если `Result` - вариант `Err`, то `unwrap` вызовет для нас макрос `panic!`. Вот пример `unwrap` в действии:

<span class="filename">Файл: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Если мы запустим этот код при отсутствии файла *hello.txt*, то увидим сообщение об ошибке из вызова `panic!` метода `unwrap`:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

Другой метод, похожий на `unwrap`, это `expect`, позволяющий указать сообщение об ошибке для макроса `panic!`. Использование `expect` вместо `unwrap` с предоставлением хорошего сообщения об ошибке выражает ваше намерение и делает более простым отслеживание источника паники. Синтаксис метода `expect` выглядит так:

<span class="filename">Файл: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

We use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!` macro. The error message used by `expect` in its call to `panic!` will be the parameter that we pass to `expect`, rather than the default `panic!` message that `unwrap` uses. Here’s what it looks like:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

In production-quality code, most Rustaceans choose `expect` rather than `unwrap` and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

### Проброс ошибок

Когда вы пишете функцию, реализация которой вызывает что-то, что может завершиться ошибкой, вместо обработки ошибки в этой функции, вы можете вернуть ошибку в вызывающий код, чтобы он мог решить, что с ней делать. Такой приём известен как *распространение ошибки* (*propagating the error*). Благодаря нему мы даём больше контроля вызывающему коду, где может быть больше информации или логики, которая диктует, как ошибка должна обрабатываться, чем было бы в месте появления этой ошибки.

Например, код программы 9-6 читает имя пользователя из файла. Если файл не существует или не может быть прочтён, то функция возвращает ошибку в код, который вызвал данную функцию.

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: A function that returns errors to the calling code using <code>match</code></span>

Эта функция может быть написана гораздо более коротким способом, но мы начнём с того, что многое сделаем вручную, чтобы изучить обработку ошибок; а в конце покажем более короткий способ. Давайте сначала рассмотрим тип возвращаемого значения: `Result<String, io::Error>`. Здесь есть возвращаемое значение функции типа `Result<T, E>` где шаблонный параметр `T` был заполнен конкретным типом `String` и шаблонный параметр `E` был заполнен конкретным типом `io::Error`.

If this function succeeds without any problems, the code that calls this function will receive an `Ok` value that holds a `String`—the username that this function read from the file. If this function encounters any problems, the calling code will receive an `Err` value that holds an instance of `io::Error` that contains more information about what the problems were. We chose `io::Error` as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the `File::open` function and the `read_to_string` method.

The body of the function starts by calling the `File::open` function. Then we handle the `Result` value with a `match` similar to the `match` in Listing 9-4. If `File::open` succeeds, the file handle in the pattern variable `file` becomes the value in the mutable variable `username_file` and the function continues. In the `Err` case, instead of calling `panic!`, we use the `return` keyword to return early out of the function entirely and pass the error value from `File::open`, now in the pattern variable `e`, back to the calling code as this function’s error value.

So if we have a file handle in `username_file`, the function then creates a new `String` in variable `username` and calls the `read_to_string` method on the file handle in `username_file` to read the contents of the file into `username`. The `read_to_string` method also returns a `Result` because it might fail, even though `File::open` succeeded. So we need another `match` to handle that `Result`: if `read_to_string` succeeds, then our function has succeeded, and we return the username from the file that’s now in `username` wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the same way that we returned the error value in the `match` that handled the return value of `File::open`. However, we don’t need to explicitly say `return`, because this is the last expression in the function.

Затем код, вызывающий этот, будет обрабатывать получение либо значения `Ok`, содержащего имя пользователя, либо значения `Err`, содержащего `io::Error`. Вызывающий код должен решить, что делать с этими значениями. Если вызывающий код получает значение `Err`, он может вызвать `panic!` и завершить работу программы, использовать имя пользователя по умолчанию или найти имя пользователя, например, не в файле. У нас недостаточно информации о том, что на самом деле пытается сделать вызывающий код, поэтому мы распространяем всю информацию об успехах или ошибках вверх, чтобы она могла обрабатываться соответствующим образом.

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.

#### Сокращение для проброса ошибок: оператор `?`

Listing 9-7 shows an implementation of `read_username_from_file` that has the same functionality as in Listing 9-6, but this implementation uses the `?` operator.

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: A function that returns errors to the calling code using the <code>?</code> operator</span>

The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` values in Listing 9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propagated to the calling code.

There is a difference between what the `match` expression from Listing 9-6 does and what the `?` operator does: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another. When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

Например, мы могли бы изменить функцию `read_username_from_file` в листинге 9-7, чтобы возвращать пользовательский тип ошибки с именем `OurError`, который мы определим. Если мы также определим `impl From<io::Error> for OurError` для создания экземпляра `OurError` из `io::Error`, то оператор `?`, вызываемый в теле `read_username_from_file`, вызовет `from` и преобразует типы ошибок без необходимости добавления дополнительного кода в функцию.

In the context of Listing 9-7, the `?` at the end of the `File::open` call will return the value inside an `Ok` to the variable `username_file`. If an error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code. The same thing applies to the `?` at the end of the `read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?`, as shown in Listing 9-8.

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: Chaining method calls after the <code>?</code> operator</span>

We’ve moved the creation of the new `String` in `username` to the beginning of the function; that part hasn’t changed. Instead of creating a variable `username_file`, we’ve chained the call to `read_to_string` directly onto the result of `File::open("hello.txt")?`. We still have a `?` at the end of the `read_to_string` call, and we still return an `Ok` value containing `username` when both `File::open` and `read_to_string` succeed rather than returning errors. The functionality is again the same as in Listing 9-6 and Listing 9-7; this is just a different, more ergonomic way to write it.

Продолжая рассматривать разные способы записи данной функции, листинг 9-9 демонстрирует способ сделать её ещё короче.

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: Using <code>fs::read_to_string</code> instead of opening and then reading the file</span>

Чтение файла в строку довольно распространённая операция, так что стандартная библиотека предоставляет удобную функцию `fs::read_to_string`, которая открывает файл, создаёт новую `String`, читает содержимое файла, размещает его в `String` и возвращает её. Конечно, использование функции `fs::read_to_string` не даёт возможности объяснить обработку всех ошибок, поэтому мы сначала изучили длинный способ.

#### Где можно использовать оператор `?`

The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on. This is because the `?` operator is defined to perform an early return of a value out of the function, in the same manner as the `match` expression we defined in Listing 9-6. In Listing 9-6, the `match` was using a `Result` value, and the early return arm returned an `Err(e)` value. The return type of the function has to be a `Result` so that it’s compatible with this `return`.

В листинге 9-10 давайте посмотрим на ошибку, которую мы получим, если воспользуемся `?` оператор в `main` функции с типом возвращаемого значения, несовместимым с типом используемого нами значения `?` на:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Listing 9-10: Attempting to use the <code>?</code> in the <code>main</code> function that returns <code>()</code> won’t compile</span>

This code opens a file, which might fail. The `?` operator follows the `Result` value returned by `File::open`, but this `main` function has the return type of `()`, not `Result`. When we compile this code, we get the following error message:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

This error points out that we’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`.

To fix the error, you have two choices. One choice is to change the return type of your function to be compatible with the value you’re using the `?` operator on as long as you have no restrictions preventing that. The other technique is to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.

Эта ошибка указывает на то, что оператор `?` разрешено использовать только в функции, которая возвращает `Result`, `Option` или другой тип, который реализует `FromResidual`. Чтобы исправить ошибку, есть два варианта. Первый - изменить тип возвращаемый из функции, чтобы он был совместим со значением, для которого вы используете оператор `?`, если у вас нет ограничений, препятствующих этому. Второй заключается в использовании `match` или одного из методов `Result<T, E>` для обработки `Result<T, E>` любым подходящим способом.

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Listing 9-11: Using the <code>?</code> operator on an <code>Option&lt;T&gt;</code> value</span>

This function returns `Option<char>` because it’s possible that there is a character there, but it’s also possible that there isn’t. This code takes the `text` string slice argument and calls the `lines` method on it, which returns an iterator over the lines in the string. Because this function wants to examine the first line, it calls `next` on the iterator to get the first value from the iterator. If `text` is the empty string, this call to `next` will return `None`, in which case we use `?` to stop and return `None` from `last_char_of_first_line`. If `text` is not the empty string, `next` will return a `Some` value containing a string slice of the first line in `text`.

Символ `?` извлекает фрагмент строки, и мы можем вызвать `chars` для этого фрагмента строки. чтобы получить итератор символов. Нас интересует последний символ в первой строке, поэтому мы вызываем `last`, чтобы вернуть последний элемент в итераторе. Вернётся `Option`, потому что возможно, что первая строка пустая - например, если `text` начинается с пустой строки, но имеет символы в других строках, как в `"\nhi"`. Однако, если в первой строке есть последний символ, он будет возвращён в варианте `Some`. Оператор `?` в середине даёт нам лаконичный способ выразить эту логику, позволяя реализовать функцию в одной строке. Если бы мы не могли использовать оператор `?` в `Option`, нам пришлось бы пришлось бы реализовать эту логику, используя больше вызовов методов или выражение `match`.

Note that you can use the `?` operator on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but you can’t mix and match. The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; in those cases, you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.

Обратите внимание, что вы можете использовать оператор `?` на `Result` в функции, которая возвращает `Result`, и вы можете использовать оператор `?` на `Option` в функции, которая возвращает `Option`, но вы не можете смешивать и сочетать их. Оператор `?` не будет автоматически преобразовывать `Result` в `Option` или наоборот; в этих случаях, вы можете использовать такие методы, как `ok` из `Result` или `ok_or` из `Option` для явного преобразования.

К счастью, `main` также может возвращать `Result<(), E>` . В листинге 9-12 используется код из листинга 9-10, но мы изменили возвращаемый тип `main` на `Result<(), Box<dyn Error>>` и добавили возвращаемое значение `Ok(())` в конец. Теперь этот код будет скомпилирован:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Листинг 9-12: Замена <code>main</code> на return <code>Result&lt;(), E&gt;</code> позволяет использовать оператор <code>?</code> оператор над значениями <code>Result</code></span>

The `Box<dyn Error>` type is a *trait object*, which we’ll talk about in the [“Using Trait Objects that Allow for Values of Different Types”](ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)<!-- ignore --> section in Chapter 17. For now, you can read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result` value in a `main` function with the error type `Box<dyn Error>` is allowed, because it allows any `Err` value to be returned early. Even though the body of this `main` function will only ever return errors of type `std::io::Error`, by specifying `Box<dyn Error>`, this signature will continue to be correct even if more code that returns other errors is added to the body of `main`.

When a `main` function returns a `Result<(), E>`, the executable will exit with a value of `0` if `main` returns `Ok(())` and will exit with a nonzero value if `main` returns an `Err` value. Executables written in C return integers when they exit: programs that exit successfully return the integer `0`, and programs that error return some integer other than `0`. Rust also returns integers from executables to be compatible with this convention.

The `main` function may return any types that implement [the `std::process::Termination` trait](../std/process/trait.Termination.html)<!-- ignore -->, which contains a function `report` that returns an `ExitCode` Consult the standard library documentation for more information on implementing the `Termination` trait for your own types.

Теперь, когда мы обсудили детали вызова `panic!` или возврата `Result`, давайте вернёмся к тому, как решить, какой из случаев подходит для какой ситуации.
