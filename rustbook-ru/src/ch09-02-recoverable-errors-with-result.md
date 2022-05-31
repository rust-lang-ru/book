## Исправимые ошибки с `Result`

Многие ошибки являются не настолько критичными, чтобы останавливать выполнение программы. Иногда, когда в функции происходит сбой, необходима просто правильная интерпретация и обработка ошибки. К примеру, при попытке открыть файл может произойти ошибка из-за отсутствия файла. Вы, возможно, захотите исправить ситуацию и создать новый файл вместо остановки программы.

Вспомните раздел ["Обработка потенциального сбоя с помощью типа `Result`"](ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type)<!-- ignore --> главы 2: мы использовали там перечисление `Result`, имеющее два варианта, `Ok` и `Err` для обработки сбоев. Само перечисление определено следующим образом:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters: we’ll discuss generics in more detail in Chapter 10. What you need to know right now is that `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant. Because `Result` has these generic type parameters, we can use the `Result` type and the functions defined on it in many different situations where the successful value and error value we want to return may differ.

Давайте вызовем функцию, которая возвращает значение `Result`, потому что может потерпеть неудачу. В листинге 9-3 мы пытаемся открыть файл.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Листинг 9-3: Открытие файла</span>

How do we know `File::open` returns a `Result`? We could look at the [standard library API documentation](../std/fs/struct.File.html#method.open)<!-- ignore -->, or we could ask the compiler! If we give `f` a type annotation that we know is *not* the return type of the function and then try to compile the code, the compiler will tell us that the types don’t match. The error message will then tell us what the type of `f` *is*. Let’s try it! We know that the return type of `File::open` isn’t of type `u32`, so let’s change the `let f` statement to this:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

Попытка компиляции выводит сообщение:

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

Ошибка говорит нам о том, что возвращаемым типом функции `File::open` является `Result<T, E>`. Типовой параметр `T` здесь равен типу успешного выполнения, `std::fs::File`, то есть дескриптору файла. Тип `E`, используемый в значении ошибки, равен `std::io::Error`.

Этот возвращаемый тип означает, что вызов `File::open` может завершиться успешно и вернуть дескриптор файла, с помощью которого можно читать из файла или писать в него. Вызов функции также может завершиться ошибкой: например, файла может не существовать или у нас может не быть прав на доступ к нему. Функция `File::open` должна иметь способ сообщить нам, был ли её вызов успешен или потерпел неудачу и одновременно возвратить либо дескриптор файла либо информацию об ошибке. Эта информация - именно то, что возвращает перечисление `Result`.

Когда вызов `File::open` успешен, значение в переменной `f` будет экземпляром `Ok`, внутри которого содержится дескриптор файла. Если вызов не успешный, значением переменной `f` будет экземпляр `Err`, который содержит больше информации о том, какая ошибка произошла.

Необходимо дописать в код листинга 9-3 выполнение разных действий в зависимости от значения, которое вернёт вызов `File::open`. Листинг 9-4 показывает один из способов обработки `Result` - пользуясь базовым инструментом языка, таким как выражение `match`, рассмотренным в Главе 6.

<span class="filename">Файл: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Листинг 9-4: Использование выражения <code>match</code> для обработки возвращаемых вариантов типа <code>Result</code></span>

Обратите внимание, что также как перечисление `Option`, перечисление `Result` и его варианты, входят в область видимости благодаря авто-импорту (prelude), поэтому не нужно указывать `Result::` перед использованием вариантов `Ok` и `Err` в ветках выражения `match`.

When the result is `Ok`, this code will return the inner `file` value out of the `Ok` variant, and we then assign that file handle value to the variable `f`. After the `match`, we can use the file handle for reading or writing.

Другая ветвь `match` обрабатывает случай, где мы получаем значение `Err` после вызова `File::open`. В этом примере мы решили вызвать макрос `panic!`. Если в нашей текущей директории нет файла с именем *hello.txt* и мы выполним этот код, то мы увидим следующее сообщение от макроса `panic!`:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Как обычно, данное сообщение точно говорит, что пошло не так.

### Обработка различных ошибок с помощью match

The code in Listing 9-4 will `panic!` no matter why `File::open` failed. However, we want to take different actions for different failure reasons: if `File::open` failed because the file doesn’t exist, we want to create the file and return the handle to the new file. If `File::open` failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to `panic!` in the same way as it did in Listing 9-4. For this we add an inner `match` expression, shown in Listing 9-5.

<span class="filename">Файл: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Листинг 9-5: Обработка различных ошибок разными способами</span>

Типом значения возвращаемого функцией `File::open` внутри `Err` варианта является `io::Error`, структура из стандартной библиотеки. Данная структура имеет метод `kind`, который можно вызвать для получения значения `io::ErrorKind`. Перечисление `io::ErrorKind` из стандартной библиотеки имеет варианты, представляющие различные типы ошибок, которые могут появиться при выполнении операций в `io` (крейте который занимается проблемами ввода/вывода данных). Вариант, который мы хотим использовать, это `ErrorKind::NotFound`. Он даёт информацию, о том, что файл который мы пытаемся открыть ещё не существует. Итак, во второй строке мы вызываем сопоставление шаблона с переменной `f` и попадаем в ветку с обработкой ошибки, но также у нас есть внутренняя проверка для сопоставления `error.kind()` ошибки.

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
>     let f = File::open("hello.txt").unwrap_or_else(|error| {
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
> Although this code has the same behavior as Listing 9-5, it doesn’t contain any `match` expressions and is cleaner to read. Come back to this example after you’ve read Chapter 13, and look up the `unwrap_or_else` method in the standard library documentation. Many more of these methods can clean up huge nested `match` expressions when you’re dealing with errors.

### Лаконичные способы обработки ошибок - `unwrap` и `expect`

Использование `match` работает достаточно хорошо, но может быть довольно многословным и не всегда хорошо передаёт смысл. Тип `Result<T, E>` имеет множество вспомогательных методов для выполнения различных, более специфических задач. Метод `unwrap` - это метод быстрого доступа к значениям, реализованный так же, как и выражение `match`, которое мы написали в Листинге 9-4. Если значение `Result` является вариантом `Ok`, `unwrap` возвращает значение внутри `Ok`. Если `Result` - вариант `Err`, то `unwrap` вызовет для нас макрос `panic!`. Вот пример `unwrap` в действии:

<span class="filename">Файл: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Если мы запустим этот код при отсутствии файла *hello.txt* , то увидим сообщение об ошибке из вызова `panic!` метода `unwrap` :

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

Другой метод, похожий на `unwrap`, это `expect`, позволяющий указать сообщение об ошибке для макроса `panic!`. Использование `expect` вместо `unwrap` с предоставлением хорошего сообщения об ошибке выражает ваше намерение и делает более простым отслеживание источника паники. Синтаксис метода `expect` выглядит так:

<span class="filename">Файл: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

Мы используем `expect` таким же образом, как и `unwrap`: чтобы вернуть дескриптор файла или вызвать макрос `panic!`. Сообщением об ошибке, которое `expect` передаст в `panic!`, будет параметр функции `expect`, а не значение по умолчанию, используемое `unwrap`. Вот как оно выглядит:

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Так как сообщение об ошибке начинается с нашего пользовательского текста: `Failed to open hello.txt`, то потом будет проще найти из какого места в коде данное сообщение приходит. Если использовать `unwrap` во множестве мест, то придётся потратить время для выяснения какой именно вызов `unwrap` вызывает "панику", так как все вызовы  <code>unwrap</code> генерируют одинаковое сообщение.

### Проброс ошибок

Когда вы пишете функцию, реализация которой вызывает что-то, что может завершиться ошибкой, вместо обработки ошибки в этой функции, вы можете вернуть ошибку в вызывающий код, чтобы он мог решить, что с ней делать. Такой приём известен как *распространение ошибки* (*propagating the error*). Благодаря нему мы даём больше контроля вызывающему коду, где может быть больше информации или логики, которая диктует, как ошибка должна обрабатываться, чем было бы в месте появления этой ошибки.

Например, код программы 9-6 читает имя пользователя из файла. Если файл не существует или не может быть прочтён, то функция возвращает ошибку в код, который вызвал данную функцию:

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Листинг 9-6: Функция, которая возвращает ошибки в вызывающий код, используя оператор <code>match</code></span>

This function can be written in a much shorter way, but we’re going to start by doing a lot of it manually in order to explore error handling; at the end, we’ll show the shorter way. Let’s look at the return type of the function first: `Result<String, io::Error>`. This means the function is returning a value of the type `Result<T, E>` where the generic parameter `T` has been filled in with the concrete type `String`, and the generic type `E` has been filled in with the concrete type `io::Error`. If this function succeeds without any problems, the code that calls this function will receive an `Ok` value that holds a `String`—the username that this function read from the file. If this function encounters any problems, the calling code will receive an `Err` value that holds an instance of `io::Error` that contains more information about what the problems were. We chose `io::Error` as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the `File::open` function and the `read_to_string` method.

The body of the function starts by calling the `File::open` function. Then we handle the `Result` value with a `match` similar to the `match` in Listing 9-4. If `File::open` succeeds, the file handle in the pattern variable `file` becomes the value in the mutable variable `f` and the function continues. In the `Err` case, instead of calling `panic!`, we use the `return` keyword to return early out of the function entirely and pass the error value from `File::open`, now in the pattern variable `e`, back to the calling code as this function’s error value.

So if we have a file handle in `f`, the function then creates a new `String` in variable `s` and calls the `read_to_string` method on the file handle in `f` to read the contents of the file into `s`. The `read_to_string` method also returns a `Result` because it might fail, even though `File::open` succeeded. So we need another `match` to handle that `Result`: if `read_to_string` succeeds, then our function has succeeded, and we return the username from the file that’s now in `s` wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the same way that we returned the error value in the `match` that handled the return value of `File::open`. However, we don’t need to explicitly say `return`, because this is the last expression in the function.

The code that calls this code will then handle getting either an `Ok` value that contains a username or an `Err` value that contains an `io::Error`. It’s up to the calling code to decide what to do with those values. If the calling code gets an `Err` value, it could call `panic!` and crash the program, use a default username, or look up the username from somewhere other than a file, for example. We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately.

Такая схема распространения ошибок настолько распространена в Rust, что Rust предоставляет оператор вопросительный знак `?` для простоты.

#### Сокращение для проброса ошибок: оператор `?`

Listing 9-7 shows an implementation of `read_username_from_file` that has the same functionality as in Listing 9-6, but this implementation uses the `?` operator.

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Листинг 9-7: Функция, которая возвращает ошибки в вызывающий код, используя оператор <code>?</code></span>

Оператор `?`, помещаемый после значения типа `Result`, работает почти таким же образом, как выражение `match`, которое мы определили для обработки значений типа `Result` в листинге 9-6. Если значение `Result` равно `Ok`, значение внутри `Ok` будет возвращено из этого выражения и программа продолжит выполнение. Если значение является `Err`, то `Err` будет возвращено из всей функции, как если бы мы использовали ключевое слово `return`, таким образом ошибка передаётся в вызывающий код.

There is a difference between what the `match` expression from Listing 9-6 does and what the `?` operator does: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another. When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as there’s an `impl From<OtherError> for ReturnedError` to define the conversion in the trait’s `from` function, the `?` operator takes care of calling the `from` function automatically.

В коде примера 9-7 оператор `?` в конце вызова функции `File::open` возвращает значения содержимого `Ok` в переменную `f`. Если же в при работе этой функции произошла ошибка, оператор `?` произведёт ранний возврат из функции со значением `Err`. То же касается `?` в конце вызова `read_to_string`.

Использование оператора `?` позволят уменьшить количество строк кода и сделать реализацию проще. Написанный в предыдущем примере код можно сделать ещё короче с помощью сокращения промежуточных переменных и конвейерного вызова нескольких методов подряд, как показано в листинге 9-8:

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: Chaining method calls after the <code>?</code> operator</span>

Мы перенесли в начало функции создание новой переменной `s` типа `String`; эта часть не изменилась. Вместо создания переменной `f` мы добавили вызов `read_to_string` непосредственно к результату `File::open("hello.txt")?`, У нас ещё есть `?` в конце вызова `read_to_string`, и мы по-прежнему возвращаем значение `Ok`, содержащее имя пользователя в `s` когда оба метода: `File::open` и `read_to_string` успешны, а не возвращают ошибки. Функциональность снова такая же, как в листинге 9-6 и листинге 9-7; это просто другой, более эргономичный способ решения той же задачи.

Продолжая рассматривать разные способы записи данной функции, листинг 9-9 демонстрирует способ сделать её ещё короче.

<span class="filename">Файл: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Листинг 9-9: Использование <code>fs::read_to_string</code> вместо открытия и чтения файла</span>

Reading a file into a string is a fairly common operation, so the standard library provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it. Of course, using `fs::read_to_string` doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.

#### Where The `?` Operator Can Be Used

The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on. This is because the `?` operator is defined to perform an early return of a value out of the function, in the same manner as the `match` expression we defined in Listing 9-6. In Listing 9-6, the `match` was using a `Result` value, and the early return arm returned an `Err(e)` value. The return type of the function has to be a `Result` so that it’s compatible with this `return`.

In Listing 9-10, let’s look at the error we’ll get if we use the `?` operator in a `main` function with a return type incompatible with the type of the value we use `?` on:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Listing 9-10: Attempting to use the <code>?</code> in the <code>main</code> function that returns <code>()</code> won’t compile</span>

This code opens a file, which might fail. The `?` operator follows the `Result` value returned by `File::open`, but this `main` function has the return type of `()`, not `Result`. When we compile this code, we get the following error message:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

This error points out that we’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`. To fix the error, you have two choices. One choice is to change the return type of your function to be compatible with the value you’re using the `?` operator on as long as you have no restrictions preventing that. The other technique is to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.

The error message also mentioned that `?` can be used with `Option<T>` values as well. As with using `?` on `Result`, you can only use `?` on `Option` in a function that returns an `Option`. The behavior of the `?` operator when called on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`: if the value is `None`, the `None` will be returned early from the function at that point. If the value is `Some`, the value inside the `Some` is the resulting value of the expression and the function continues. Listing 9-11 has an example of a function that finds the last character of the first line in the given text:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Listing 9-11: Using the <code>?</code> operator on an <code>Option&lt;T&gt;</code> value</span>

This function returns `Option<char>` because it’s possible that there is a character there, but it’s also possible that there isn’t. This code takes the `text` string slice argument and calls the `lines` method on it, which returns an iterator over the lines in the string. Because this function wants to examine the first line, it calls `next` on the iterator to get the first value from the iterator. If `text` is the empty string, this call to `next` will return `None`, in which case we use `?` to stop and return `None` from `last_char_of_first_line`. If `text` is not the empty string, `next` will return a `Some` value containing a string slice of the first line in `text`.

Символ `?` извлекает фрагмент строки, и мы можем вызвать `chars` для этого фрагмента строки. чтобы получить итератор символов. Нас интересует последний символ в первой строке, поэтому мы вызываем `last`, чтобы вернуть последний элемент в итераторе. Вернётся `Option`, потому что возможно, что первая строка пустая - например, если `text` начинается с пустой строки, но имеет символы в других строках, как в `"\nhi"`. Однако, если в первой строке есть последний символ, он будет возвращён в варианте `Some`. Оператор `?` в середине даёт нам лаконичный способ выразить эту логику, позволяя реализовать функцию в одной строке. Если бы мы не могли использовать оператор `?` в `Option`, нам пришлось бы пришлось бы реализовать эту логику, используя больше вызовов методов или выражение `match`.

Note that you can use the `?` operator on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but you can’t mix and match. The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; in those cases, you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.

So far, all the `main` functions we’ve used return `()`. The `main` function is special because it’s the entry and exit point of executable programs, and there are restrictions on what its return type can be for the programs to behave as expected.

Luckily, `main` can also return a `Result<(), E>`. Listing 9-12 has the code from Listing 9-10 but we’ve changed the return type of `main` to be `Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end. This code will now compile:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Listing 9-12: Changing <code>main</code> to return <code>Result&lt;(), E&gt;</code> allows the use of the <code>?</code> operator on <code>Result</code> values</span>

The `Box<dyn Error>` type is a *trait object*, which we’ll talk about in the [“Using Trait Objects that Allow for Values of Different Types”](ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)<!-- ignore --> section in Chapter 17. For now, you can read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result` value in a `main` function with the error type `Box<dyn Error>` is allowed, because it allows any `Err` value to be returned early.

When a `main` function returns a `Result<(), E>`, the executable will exit with a value of `0` if `main` returns `Ok(())` and will exit with a nonzero value if `main` returns an `Err` value. Executables written in C return integers when they exit: programs that exit successfully return the integer `0`, and programs that error return some integer other than `0`. Rust also returns integers from executables to be compatible with this convention.

The `main` function may return any types that implement [the `std::process::Termination` trait](../std/process/trait.Termination.html)<!-- ignore -->. As of this writing, the `Termination` trait is an unstable feature only available in Nightly Rust, so you can’t yet implement it for your own types in Stable Rust, but you might be able to someday!

Теперь, когда мы обсудили детали вызова `panic!` или возврата `Result`, давайте вернёмся к тому, как решить, какой из случаев подходит для какой ситуации.
