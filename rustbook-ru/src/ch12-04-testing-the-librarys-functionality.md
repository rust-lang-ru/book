## Развитие функциональности библиотеки разработкой на основе тестов

Теперь, когда мы извлекли логику в *src/lib.rs* и оставили разбор аргументов командной строки и обработку ошибок в *src/main.rs*, стало гораздо проще писать тесты для основной функциональности нашего кода. Мы можем вызывать функции напрямую с различными аргументами и проверить возвращаемые значения без необходимости вызова нашего двоичного файла из командной строки.

In this section, we’ll add the searching logic to the `minigrep` program using the test-driven development (TDD) process with the following steps:

1. Напишите тест, который не прошёл и запустите его, чтобы убедиться, что он не прошёл по той причине, которую вы ожидаете.
2. Пишите или изменяйте ровно столько кода, чтобы успешно выполнился новый тест.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Повторите с шага 1!

Though it’s just one of many ways to write software, TDD can help drive code design. Writing the test before you write the code that makes the test pass helps to maintain high test coverage throughout the process.

Мы протестируем реализацию функциональности, которая делает поиск строки запроса в содержимом файла и создание списка строк, соответствующих запросу. Мы добавим эту функциональность в функцию под названием `search` .

### Написание теста с ошибкой

Because we don’t need them anymore, let’s remove the `println!` statements from *src/lib.rs* and *src/main.rs* that we used to check the program’s behavior. Then, in *src/lib.rs*, add a `tests` module with a test function, as we did in [Chapter 11](ch11-01-writing-tests.html#the-anatomy-of-a-test-function)<!-- ignore -->. The test function specifies the behavior we want the `search` function to have: it will take a query and the text to search, and it will return only the lines from the text that contain the query. Listing 12-15 shows this test, which won’t compile yet.

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">Листинг 12-15. Создание теста с ошибкой для функции <code>search</code>, которую мы хотели бы получить</span>

Этот тест ищет строку `"duct"`. Текст, который мы ищем состоит из трёх строк, только одна из которых содержит `"duct"` (обратите внимание, что обратная косая черта после открывающей двойной кавычки говорит Rust не помещать символ новой строки в начало содержимого этого строкового литерала). Мы проверяем, что значение, возвращаемое функцией `search`, содержит только ожидаемую нами строку.

We aren’t yet able to run this test and watch it fail because the test doesn’t even compile: the `search` function doesn’t exist yet! In accordance with TDD principles, we’ll add just enough code to get the test to compile and run by adding a definition of the `search` function that always returns an empty vector, as shown in Listing 12-16. Then the test should compile and fail because an empty vector doesn’t match a vector containing the line `"safe, fast, productive."`

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">Листинг 12-16. Определение функции <code>search</code>, достаточное, чтобы тест скомпилировался</span>

Notice that we need to define an explicit lifetime `'a` in the signature of `search` and use that lifetime with the `contents` argument and the return value. Recall in [Chapter 10](ch10-03-lifetime-syntax.html)<!-- ignore --> that the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain string slices that reference slices of the argument `contents` (rather than the argument `query`).

Другими словами, мы говорим Rust, что данные, возвращаемые функцией `search`, будут жить до тех пор, пока живут данные, переданные в функцию `search` через аргумент `contents`. Это важно! Чтобы ссылки были действительными, данные, на которые ссылаются *с помощью* срезов тоже должны быть действительными; если компилятор предполагает, что мы делаем строковые срезы переменной `query`, а не переменной `contents`, он неправильно выполнит проверку безопасности.

Если мы забудем аннотации времени жизни и попробуем скомпилировать эту функцию, то получим следующую ошибку:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust can’t possibly know which of the two arguments we need, so we need to tell it explicitly. Because `contents` is the argument that contains all of our text and we want to return the parts of that text that match, we know `contents` is the argument that should be connected to the return value using the lifetime syntax.

Other programming languages don’t require you to connect arguments to return values in the signature, but this practice will get easier over time. You might want to compare this example with the [“Validating References with Lifetimes”](ch10-03-lifetime-syntax.html#validating-references-with-lifetimes)<!-- ignore --> section in Chapter 10.

Запустим тест:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

Отлично. Наш тест не сработал, как мы и ожидали. Давайте сделаем так, чтобы он срабатывал!

### Написание кода для прохождения теста

Сейчас наш тест не проходит, потому что мы всегда возвращаем пустой вектор. Чтобы исправить это и реализовать `search`, наша программа должна выполнить следующие шаги:

- Итерироваться по каждой строке содержимого.
- Проверить, содержит ли данная строка искомую.
- Если это так, добавить её в список значений, которые мы возвращаем.
- Если это не так, ничего не делать.
- Вернуть список результатов.

Давайте проработаем каждый шаг, начиная с перебора строк.

#### Перебор строк с помощью метода `lines`

В Rust есть полезный метод для построчной итерации строк, удобно названый `lines`, как показано в листинге 12-17. Обратите внимание, код пока не компилируется.

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">Листинг 12-17: Итерация по каждой строке из <code>contents</code></span>

Метод `lines` возвращает итератор. Мы подробно поговорим об итераторах в [Главе 13](ch13-02-iterators.html)<!-- ignore -->, но вспомните, что вы видели этот способ использования итератора в [Листинге 3-5](ch03-05-control-flow.html#looping-through-a-collection-with-for)<!-- ignore -->, где мы использовали цикл `for` с итератором, чтобы выполнить некоторый код для каждого элемента в коллекции.

#### Поиск в каждой строке текста запроса

Далее мы проверяем, содержит ли текущая строка нашу искомую строку. К счастью, у строк есть полезный метод `contains`, который именно это и делает! Добавьте вызов метода `contains` в функции `search`, как показано в листинге 12-18. Обратите внимание, что это все ещё не компилируется.

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">Листинг 12-18. Добавление проверки, содержится ли <code>query</code> в строке</span>

At the moment, we’re building up functionality. To get it to compile, we need to return a value from the body as we indicated we would in the function signature.

#### Сохранение совпавшей строки

To finish this function, we need a way to store the matching lines that we want to return. For that, we can make a mutable vector before the `for` loop and call the `push` method to store a `line` in the vector. After the `for` loop, we return the vector, as shown in Listing 12-19.

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">Листинг 12-19. Сохранение совпадающих строк, чтобы вернуть их</span>

Теперь функция `search` должна возвратить только строки, содержащие `query`, и тест должен пройти. Запустим его:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Наш тест пройден, значит он работает!

На этом этапе мы могли бы рассмотреть возможности изменения реализации функции поиска, сохраняя прохождение тестов и поддерживая имеющуюся функциональность. Код в функции поиска не так уж плох, но он не использует некоторые полезные функции итераторов. Вернёмся к этому примеру в [главе 13](ch13-02-iterators.html)<!-- ignore -->, где будем исследовать итераторы подробно, и посмотрим как его улучшить.

#### Использование функции `search` в функции `run`

Теперь, когда функция `search` работает и протестирована, нужно вызвать `search` из нашей функции `run`. Нам нужно передать значение `config.query` и `contents`, которые `run` читает из файла, в функцию `search`. Тогда `run` напечатает каждую строку, возвращаемую из `search` :

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

Мы по-прежнему используем цикл `for` для возврата каждой строки из функции `search` и её печати.

Теперь вся программа должна работать! Давайте попробуем сначала запустить её со словом «frog», которое должно вернуть только одну строчку из стихотворения Эмили Дикинсон:

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Здорово! Теперь давайте попробуем слово, которое будет соответствовать нескольким строкам, например «body»:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

И наконец, давайте удостоверимся, что мы не получаем никаких строк, когда ищем слово, отсутствующее в стихотворении, например «monomorphization»:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Отлично! Мы создали собственную мини-версию классического инструмента и научились тому, как структурировать приложения. Мы также немного узнали о файловом вводе и выводе, временах жизни, тестировании и разборе аргументов командной строки.

Чтобы завершить этот проект, мы кратко продемонстрируем пару вещей: как работать с переменными окружения и как печатать в стандартный поток ошибок, обе из которых полезны при написании консольных программ.
