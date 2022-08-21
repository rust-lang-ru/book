## Контролирование хода выполнения тестов

Just as `cargo run` compiles your code and then runs the resulting binary, `cargo test` compiles your code in test mode and runs the resulting test binary. The default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results. You can, however, specify command line options to change this default behavior.

Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary. Running `cargo test --help` displays the options you can use with `cargo test`, and running `cargo test -- --help` displays the options you can use after the separator.

### Выполнение тестов параллельно или последовательно

When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster and you get feedback quicker. Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

For example, say each of your tests runs some code that creates a file on disk named *test-output.txt* and writes some data to that file. Then each test reads the data in that file and asserts that the file contains a particular value, which is different in each test. Because the tests run at the same time, one test might overwrite the file in the time between another test writing and reading the file. The second test will then fail, not because the code is incorrect but because the tests have interfered with each other while running in parallel. One solution is to make sure each test writes to a different file; another solution is to run the tests one at a time.

Если вы не хотите запускать тесты параллельно или хотите более детальный контроль над количеством используемых потоков, можно установить флаг `--test-threads` и то количество потоков, которое вы хотите использовать для теста. Взгляните на следующий пример:

```console
$ cargo test -- --test-threads=1
```

Мы устанавливаем количество тестовых потоков равным `1` , указывая программе не использовать параллелизм. Выполнение тестов с использованием одного потока займёт больше времени, чем их параллельное выполнение, но тесты не будут мешать друг другу, если они совместно используют состояние.

### Демонстрация результатов работы функции

По умолчанию, если тест пройден, система управления запуска тестов блокирует вывод на печать, т.е. если вы вызовете макрос `println!` внутри кода теста и тест будет пройден, вы не увидите вывода на консоль результатов вызова `println!`. Если же тест не был пройден, все информационные сообщение, а также описание ошибки будет выведено на консоль.

Например, в коде (11-10) функция выводит значение параметра с поясняющим текстовым сообщением, а также возвращает целочисленное константное значение <code>10</code>. Далее следует тест, который имеет правильный входной параметр и тест, который имеет ошибочный входной параметр:

<span class="filename">Файл: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

<span class="caption">Листинг 11-10: Тест функции, которая использует макрос println!</span>

Результат вывода на консоль команды `cargo test`:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Обратите внимание, что нигде в этом выводе мы не видим сообщения `I got the value 4` , которое печатается при выполнении пройденного теста. Этот вывод был записан. Результат неудачного теста, `I got the value 8` , появляется в разделе итоговых результатов теста, который также показывает причину неудачного теста.

Если мы хотим видеть напечатанные результаты прохождения тестов, мы можем сказать Rust, чтобы он также показывал результаты успешных тестов с помощью `--show-output`.

```console
$ cargo test -- --show-output
```

Когда мы снова запускаем тесты из Листинга 11-10 с флагом `--show-output` , мы видим следующий результат:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Запуск подмножества тестов по имени

Бывают случаи, когда в запуске всех тестов нет необходимости и нужно запустить только несколько тестов. Если вы работаете над функцией и хотите запустить тесты, которые исследуют её работу - это было бы удобно. Вы можете это сделать, используя команду `cargo test`, передав в качестве аргумента имена тестов.

Для демонстрации, как запустить группу тестов, мы создадим группу тестов для функции `add_two` function, как показано в Листинге 11-11, и постараемся выбрать какие из них запускать.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<span class="caption">Листинг 11-11: Три теста с различными именами</span>

Если вы выполните команду `cargo test` без уточняющих аргументов, все тесты выполнятся параллельно:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Запуск одного теста

Мы можем запустить один тест с помощью указания его имени в команде `cargo test`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Был запущен только тест с названием `one_hundred`; два других теста не соответствовали этому названию. Результаты теста с помощью вывода `2 filtered out` дают нам понять, что у нас было больше тестов, но они не были запущены.

Таким образом мы не можем указать имена нескольких тестов; будет использоваться только первое значение, указанное для `cargo test` . Но есть способ запустить несколько тестов.

#### Использование фильтров для запуска нескольких тестов

Мы можем указать часть имени теста, и будет запущен любой тест, имя которого соответствует этому значению. Например, поскольку имена двух наших тестов содержат `add`, мы можем запустить эти два, запустив `cargo test add`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Эта команда запускала все тесты с `add` в имени и отфильтровывала тест с именем `one_hundred` . Также обратите внимание, что модуль, в котором появляется тест, становится частью имени теста, поэтому мы можем запускать все тесты в модуле, фильтруя имя модуля.

### Игнорирование тестов

Бывает, что некоторые тесты требуют продолжительного времени для своего исполнения, и вы хотите исключить их из исполнения при запуске `cargo test`. Вместо перечисления в командной строке всех тестов, которые вы хотите запускать, вы можете аннотировать тесты, требующие много времени для прогона, атрибутом `ignore`, чтобы исключить их, как показано здесь:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```

После `#[test]` мы добавляем строку `#[ignore]` в тест, который хотим исключить. Теперь, когда мы запускаем наши тесты, `it_works` запускается, а `expensive_test` игнорируется:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

Функция `expensive_test` помечена как `ignored`. Если вы хотите выполнить только проигнорированные тесты, вы можете воспользоваться командой `cargo test -- --ignored`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Управляя тем, какие тесты запускать, вы можете быть уверены, что результаты вашего `cargo test` будут быстрыми. Когда вы дойдёте до момента, где имеет смысл проверить результаты тестов `ignored`, и у вас есть время дождаться их результатов, вы можете запустить их с помощью `cargo test -- --ignored`. Если вы хотите запустить все тесты независимо от того, игнорируются они или нет, выполните `cargo test -- --include-ignored`.
