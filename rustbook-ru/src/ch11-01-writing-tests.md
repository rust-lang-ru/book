## Как писать тесты

Тесты - это функции Rust, которые проверяют, что не тестовый код работает ожидаемым образом. Содержимое тестовых функций обычно выполняет следующие три действия:

1. Установка любых необходимых данных или состояние.
2. Запуск кода, который вы хотите проверить.
3. Утверждение, что результаты являются теми, которые вы ожидаете.

Let’s look at the features Rust provides specifically for writing tests that take these actions, which include the `test` attribute, a few macros, and the `should_panic` attribute.

### Структура тестирующей функции

В простейшем случае в Rust тест - это функция, аннотированная атрибутом `test`. Атрибуты представляют собой метаданные о фрагментах кода Rust; один из примеров атрибут `derive`, который мы использовали со структурами в главе 5. Чтобы изменить функцию в тестирующую функцию добавьте `#[test]` в строку перед `fn` . Когда вы запускаете тесты командой `cargo test`, Rust создаёт бинарный модуль выполняющий функции аннотированные атрибутом `test` и сообщающий о том, прошла успешно или не прошла каждая тестирующая функция.

When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us. This module helps you start writing your tests so you don’t have to look up the exact structure and syntax of test functions every time you start a new project. You can add as many additional test functions and as many test modules as you want!

We’ll explore some aspects of how tests work by experimenting with the template test generated for us without actually testing any code. Then we’ll write some real-world tests that call some code that we’ve written and assert that its behavior is correct.

Давайте создадим новый проект библиотеки под названием `adder` :

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

Содержимое файла *src/lib.rs* вашей библиотеки `adder` должно выглядеть как в листинге 11-1.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">Listing 11-1: The test module and function generated automatically by <code>cargo new</code></span>

Сейчас проигнорируем первые две строчки кода и сосредоточимся на функции, чтобы увидеть как она работает. Обратите внимание на синтаксис аннотации `#[test]` перед ключевым словом `fn`. Этот атрибут сообщает компилятору, что это является заголовком тестирующей функции, так что функционал запускающий тесты на выполнение теперь знает, что это тестирующая функция. Также в составе модуля тестов `tests` могут быть вспомогательные функции, помогающие настроить и выполнить общие подготовительные операции, поэтому специальная аннотация важна для указания объявления функций тестами с использованием атрибута `#[test]`.

The function body uses the `assert_eq!` macro to assert that 2 + 2 equals 4. This assertion serves as an example of the format for a typical test. Let’s run it to see that this test passes.

Команда `cargo test` выполнит все тесты в выбранном проекте и сообщит о результатах как в листинге 11-2:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">Listing 11-2: The output from running the automatically generated test</span>

Cargo скомпилировал и выполнил тест. После строк `Compiling`, `Finished` и `Running` мы видим строку `running 1 test`. Следующая строка показывает имя созданной тест функции  с названием `it_works` и результат её выполнения - `ok`. Далее вы видите обобщённую информации о работе всех тестов. Текст `test result: ok.` означает, что все тесты пройдены успешно и часть вывода `1 passed; 0 failed` сообщает общее количество тестов, которые прошли или были ошибочными.

Поскольку у нас нет тестов, которые мы пометили как игнорируемые, в сводке отображается `0 ignored`. Мы также не отфильтровывали тесты для выполнения, поэтому конец сводки пишет `0 filtered out`. Мы поговорим про игнорирование и фильтрацию тестов в следующем разделе [“Controlling How Tests Are Run.”](ch11-02-running-tests.html#controlling-how-tests-are-run)<comment></comment>

The `0 measured` statistic is for benchmark tests that measure performance. Benchmark tests are, as of this writing, only available in nightly Rust. See [the documentation about benchmark tests](../unstable-book/library-features/test.html) to learn more.

Следующая часть вывода тестов начинается с `Doc-tests adder` - это информация о тестах в документации. У нас пока нет тестов документации, но Rust может компилировать любые примеры кода, которые находятся в API документации. Такая возможность помогает поддерживать документацию и код в синхронизированном состоянии. Мы поговорим о написании тестов документации в секции [“Комментарии документации как тесты”](ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)<!-- <!--  --> --> главы 14. Пока просто проигнорируем часть `Doc-tests` вывода.

Давайте поменяем название нашего теста и посмотрим что же измениться в строке вывода. Назовём нашу функцию `it_works` другим именем - `exploration`:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Then run `cargo test` again. The output now shows `exploration` instead of `it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Добавим ещё один тест, но в этот раз специально сделаем так, чтобы этот новый тест не отработал. Тест терпит неудачу, когда что-то паникует в тестируемой функции. Каждый тест запускается в новом потоке и когда главный поток видит, что тестовый поток упал, то помечает тест как завершившийся аварийно. Мы говорили о простейшем способе вызвать панику в главе 9, используя для этого известный макрос `panic!`. Введём код тест функции `another`, как в файле *src/lib.rs* из листинга 11-3.

<span class="filename">Файл: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">Listing 11-3: Adding a second test that will fail because we call the <code>panic!</code> macro</span>

Run the tests again using `cargo test`. The output should look like Listing 11-4, which shows that our `exploration` test passed and `another` failed.

```text
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">Листинг 11-4: Результаты выполнения тестов, когда один пройден, а второй нет</span>

Вместо `ok`, строка `test tests::another` сообщает `FAILED`. У нас есть два новых раздела между результатами и итогами. Первый раздел показывает детальную причину ошибки каждого теста. В данном случае тест `another` не сработал, потому что `panicked at 'Make this test fail'`, произошло в строке 10 файла *src/lib.rs*. В следующем разделе перечисляют имена всех не пройденных тестов, что удобно, когда тестов очень много и есть много деталей про аварийное завершение. Мы можем использовать имя не пройденного теста для его дальнейшей отладки; мы больше поговорим о способах запуска тестов в разделе [“Управление запуска тестов”](ch11-02-running-tests.html#controlling-how-tests-are-run)<!-- <!--  --> -->.

Итоговая строка отображается в конце: общий результат нашего тестирования `FAILED`. У нас один тест пройден и один тест завершён аварийно.

Now that you’ve seen what the test results look like in different scenarios, let’s look at some macros other than `panic!` that are useful in tests.

### Проверка результатов с помощью макроса `assert!`

Макрос `assert!` доступен из стандартной библиотеки и является удобным, когда вы хотите проверить что некоторое условие в тесте вычисляется в значение `true`. Внутри макроса `assert!` переданный аргумент вычисляется в логическое значение. Если оно `true`, то `assert!` в тесте ничего не делает и он считается пройденным. Если же значение вычисляется в `false`, то макрос  `assert!`  вызывает макрос `panic!`, что делает тест аварийным. Использование макроса `assert!` помогает проверить, что код функционирует как ожидалось.

В главе 5, листинга 5-15, мы использовали структуру `Rectangle` и метод `can_hold`, который повторён в листинге 11-5. Давайте поместим этот код в файл *src/lib.rs* и напишем несколько тестов для него используя `assert!` макрос.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">Listing 11-5: Using the <code>Rectangle</code> struct and its <code>can_hold</code> method from Chapter 5</span>

The `can_hold` method returns a Boolean, which means it’s a perfect use case for the `assert!` macro. In Listing 11-6, we write a test that exercises the `can_hold` method by creating a `Rectangle` instance that has a width of 8 and a height of 7 and asserting that it can hold another `Rectangle` instance that has a width of 5 and a height of 1.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">Листинг 11-6: Теста для метода <code>can_hold</code>, который проверяет что больший прямоугольник действительно может содержать меньший</span>

Также, в модуле `tests` обратите внимание на новую добавленную строку `use super::*;`. Модуль `tests` является обычным и подчиняется тем же правилам видимости, которые мы обсуждали в главе 7 [“Пути для ссылки на элементы внутри дерева модуля”](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)<!-- <!--  --> -->. Так как этот модуль `tests` является внутренним, нужно подключить тестируемый код из внешнего модуля в область видимости внутреннего модуля с тестами. Для этого используется глобальное подключения, так что все что определено во внешнем модуле становится доступным внутри `tests` модуле.

We’ve named our test `larger_can_hold_smaller`, and we’ve created the two `Rectangle` instances that we need. Then we called the `assert!` macro and passed it the result of calling `larger.can_hold(&smaller)`. This expression is supposed to return `true`, so our test should pass. Let’s find out!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

It does pass! Let’s add another test, this time asserting that a smaller rectangle cannot hold a larger rectangle:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Because the correct result of the `can_hold` function in this case is `false`, we need to negate that result before we pass it to the `assert!` macro. As a result, our test will pass if `can_hold` returns `false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Two tests that pass! Now let’s see what happens to our test results when we introduce a bug in our code. Let’s change the implementation of the `can_hold` method by replacing the greater than sign with a less than sign when it compares the widths:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Запуск тестов теперь производит следующее:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Our tests caught the bug! Because `larger.width` is 8 and `smaller.width` is 5, the comparison of the widths in `can_hold` now returns `false`: 8 is not less than 5.

### Проверка на равенство с помощью макросов `assert_eq!` и `assert_ne!`

A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they’re equal. You could do this using the `assert!` macro and passing it an expression using the `==` operator. However, this is such a common test that the standard library provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively. They’ll also print the two values if the assertion fails, which makes it easier to see *why* the test failed; conversely, the `assert!` macro only indicates that it got a `false` value for the `==` expression, not the values that lead to the `false` value.

В листинге 11-7, мы напишем функцию `add_two`, которая прибавляет к входному параметру `2` и возвращает значение. Затем, протестируем эту функцию с помощью макроса `assert_eq!`:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">Listing 11-7: Testing the function <code>add_two</code> using the <code>assert_eq!</code> macro</span>

Проверим, что тесты проходят!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

Первый аргумент, который мы передаём в макрос `assert_eq!` число `4` чей результат вызова равен `add_two(2)` . Строка для этого теста - `test tests::it_adds_two ... ok` , а текст `ok` означает, что наш тест пройден!

Let’s introduce a bug into our code to see what it looks like when a test that uses `assert_eq!` fails. Change the implementation of the `add_two` function to instead add `3`:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Попробуем выполнить данный тест ещё раз:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Наш тест нашёл ошибку! Тест `it_adds_two` не выполнился, отображается сообщение `assertion failed: `(left == right)`` и показывает, что `left` было `4`, а `right` было `5`. Это сообщение полезно и помогает начать отладку: это означает `left` аргумент `assert_eq!` имел значение `4`, но `right` аргумент для вызова `add_two(2)` был со значением `5`.

Note that in some languages and test frameworks, the parameters to the functions that assert two values are equal are called `expected` and `actual`, and the order in which we specify the arguments matters. However, in Rust, they’re called `left` and `right`, and the order in which we specify the value we expect and the value that the code under test produces doesn’t matter. We could write the assertion in this test as `assert_eq!(add_two(2), 4)`, which would result in a failure message that displays `assertion failed: `(left == right)`` and that `left` was `5` and `right` was `4`.

The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value *will* be, but we know what the value definitely *won’t* be if our code is functioning as we intend. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. All the primitive types and most of the standard library types implement these traits. For structs and enums that you define, you’ll need to implement `PartialEq` to assert that values of those types are equal or not equal. You’ll need to implement `Debug` to print the values when the assertion fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See Appendix C, [“Derivable Traits,”](appendix-03-derivable-traits.html)<!-- ignore --> for more details about these and other derivable traits.

### Создание сообщений об ошибках

You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any arguments specified after the one required argument to `assert!` or the two required arguments to `assert_eq!` and `assert_ne!` are passed along to the `format!` macro (discussed in Chapter 8 in the <a href="ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro" data-md-type="link">“Concatenation with the <code data-md-type="codespan">+</code> Operator or the `format!` Macro”</a><!-- ignore --> section), so you can pass a format string that contains `{}` placeholders and values to go in those placeholders. Custom messages are useful to document what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

Например, есть функция, которая приветствует человека по имени и мы хотим протестировать эту функцию. Мы хотим чтобы передаваемое ей имя выводилось в консоль:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

The requirements for this program haven’t been agreed upon yet, and we’re pretty sure the `Hello` text at the beginning of the greeting will change. We decided we don’t want to have to update the test when the requirements change, so instead of checking for exact equality to the value returned from the `greeting` function, we’ll just assert that the output contains the text of the input parameter.

Let’s introduce a bug into this code by changing `greeting` to not include `name` to see what this test failure looks like:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Запуск этого теста выводит следующее:

```console
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs:here}}
```

Сообщение содержит лишь информацию о том что сравнение не было успешным и в какой строке это произошло. В данном случае, более полезный текст сообщения был бы, если бы также выводилось значение из функции `greeting`. Изменим тестирующую функцию так, чтобы выводились пользовательское сообщение форматированное строкой с заменителем и фактическими данными из кода `greeting` :

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

После того, как выполним тест ещё раз мы получим подробное сообщение об ошибке:

```console
---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not
contain name, value was `Hello!`', src/lib.rs:12:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.

### Проверка с помощью макроса `should_panic`

In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles error conditions as we expect. For example, consider the `Guess` type that we created in Chapter 9, Listing 9-10. Other code that uses `Guess` depends on the guarantee that `Guess` instances will contain only values between 1 and 100. We can write a test that ensures that attempting to create a `Guess` instance with a value outside that range panics.

Реализуем это с помощью другого атрибута тест функции `#[should_panic]`. Этот атрибут сообщает системе тестирования, что тест проходит, когда метод генерирует ошибку. Если ошибка не генерируется - тест считается не пройденным.

Листинг 11-8 показывает тест, который проверяет, что условия ошибки `Guess::new` произойдут, когда мы их ожидаем их.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">Listing 11-8: Testing that a condition will cause a <code>panic!</code></span>

Атрибут `#[should_panic]` следует после `#[test]` и до объявления текстовой функции. Посмотрим на вывод результата, когда тест проходит:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Looks good! Now let’s introduce a bug in our code by removing the condition that the `new` function will panic if the value is greater than 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Когда мы запустим тест в листинге 11-8, он потерпит неудачу:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

Мы получаем не очень полезное сообщение в этом случае, но когда мы смотрим на тестирующую функцию, мы видим, что она `#[should_panic]`. Аварийное выполнение, которое мы получили означает, что код в тестирующей функции не вызвал паники.

Tests that use `should_panic` can be imprecise because they only indicate that the code has caused some panic. A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting to happen. To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute. The test harness will make sure that the failure message contains the provided text. For example, consider the modified code for `Guess` in Listing 11-9 where the `new` function panics with different messages depending on whether the value is too small or too large.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">Listing 11-9: Testing that a condition will cause a <code>panic!</code> with a particular panic message</span>

This test will pass because the value we put in the `should_panic` attribute’s `expected` parameter is a substring of the message that the `Guess::new` function panics with. We could have specified the entire panic message that we expect, which in this case would be `Guess value must be less than or equal to 100, got 200.` What you choose to specify in the expected parameter for `should_panic` depends on how much of the panic message is unique or dynamic and how precise you want your test to be. In this case, a substring of the panic message is enough to ensure that the code in the test function executes the `else if value > 100` case.

Чтобы увидеть, что происходит, когда тест `should_panic` не успешно завершается с сообщением `expected`, давайте снова внесём ошибку в наш код, поменяв местами `if value < 1` и `else if value > 100` блоки:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

На этот раз, когда мы выполним `should_panic` тест, он потерпит неудачу:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

The failure message indicates that this test did indeed panic as we expected, but the panic message did not include the expected string `'Guess value must be less than or equal to 100'`. The panic message that we did get in this case was `Guess value must be greater than or equal to 1, got 200.` Now we can start figuring out where our bug is!

### Использование `Result<T, E>` в тестах

Пока что мы написали тесты, которые паникуют, когда терпят неудачу. Мы также можем написать тесты которые используют `Result<T, E>`! Вот тест из листинга 11-1, переписанный для с использованием `Result<T, E>` и возвращающий `Err` вместо паники:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

The `it_works` function now has a return type, `Result<(), String>`. In the body of the function, rather than calling the `assert_eq!` macro, we return `Ok(())` when the test passes and an `Err` with a `String` inside when the test fails.

Writing tests so they return a `Result<T, E>` enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.

You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. Instead, you should return an `Err` value directly when the test should fail.

Now that you know several ways to write tests, let’s look at what is happening when we run our tests and explore the different options we can use with `cargo test`.
