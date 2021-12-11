## Функции

Функции широко распространены в коде Rust. Вы уже познакомились с одной из самых важных функций в языке: функцией `main`, которая является точкой входа большинства программ. Вы также видели ключевое слово `fn`, позволяющее объявлять новые функции.

Код Rust использует *змеиный регистр (snake case)* как основной стиль для имен функций и переменных, в котором все буквы строчные, а символ подчеркивания разделяет слова. Вот программа, содержащая пример определения функции:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Для определения функции в Rust необходимо указать `fn`, за которым следует имя функции и набор круглых скобок. Фигурные скобки указывают компилятору, где начинается и заканчивается тело функции.

Мы можем вызвать любую определенную нами функцию, указав ее имя, сопровождаемое набором круглых скобок. Так как `another_function` определена в программе, её можно вызвать из функции `main`. Обратите внимание, в исходном коде мы определили `another_function` *после* функции `main`, но можно было бы определить ее и до. Rust неважно, где вы определяете свои функции, главное, чтобы они были где-то определены.

Создадим новый бинарный проект с названием *functions* для дальнейшего изучения функций. Поместите пример `another_function` в файл *src/main.rs* и запустите его. Вы должны увидеть следующий вывод:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Строки выполняются в том порядке, в котором они расположены в функции `main`. Сначала печатается сообщение "Hello, world!", а затем вызывается `another_function`, которая также печатает сообщение.

### Параметры функции

Можно определить функции с *параметрами*, которые являются специальными переменными, входящими в сигнатуру функции. Когда функция имеет параметры, вы можете предоставить ей конкретные значения этих параметров. С технической точки зрения конкретные значения называются *аргументами*, но в неформальной беседе люди обычно используют слова *параметр* и *аргумент* как взаимозаменяемые применительно к переменным в определении функции или конкретным значениям, передаваемым при вызове функции.

В этой версии `another_function` мы добавляем параметр:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Попробуйте запустить эту программу. Должны получить следующий результат:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

Объявление `another_function` имеет один параметр с именем `x`. Тип переменной `x` задан как `i32`. Когда мы передаем `5` в `another_function`, макрос `println!` помещает `5` на место пары фигурных скобок в строке форматирования.

В сигнатурах функций вы *должны* объявить тип каждого параметра. Требование аннотаций типов в определениях функций - это намеренное решение в дизайне Rust, позволяющее компилятору дальше в коде их почти никогда не запрашивать, чтобы понять, какой тип имеется в виду.

При определении нескольких параметров, разделяйте объявления параметров запятыми, как показано ниже:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Этот пример создает функцию под именем `print_labeled_measurement` с двумя параметрами. Первый параметр называется `value` с типом `i32`. Второй называется `unit_label` и имеет тип `char`. Затем функция печатает текст, содержащий `value` и `unit_label`.

Попробуем запустить этот код. Замените текущую программу проекта *functions* в файле *src/main.rs* на предыдущий пример и запустите его с помощью `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Поскольку мы вызвали функцию с `5` в качестве значения для `value` и `'h'` в качестве значения для `unit_label`, вывод программы содержит эти значения.

### Операторы и выражения

Тела функций состоят из ряда операторов, необязательно заканчивающихся выражением. До сих пор функции, которые мы рассматривали, не включали завершающее выражение, но вы видели выражение как часть оператора. Поскольку Rust является языком, основанным на выражениях, это важное различие необходимо понимать. В других языках таких различий нет, поэтому давайте рассмотрим, что такое операторы и выражения, и как их различия влияют на тела функций.

*Statements* are instructions that perform some action and do not return a value. *Expressions* evaluate to a resulting value. Let’s look at some examples.

We’ve actually already used statements and expressions. Creating a variable and assigning a value to it with the `let` keyword is a statement. In Listing 3-1, `let y = 6;` is a statement.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Листинг 3-1: Объявление функции <code>main</code> включающей один оператор</span>

Определение функции также является оператором. Весь предыдущий пример тоже является оператором.

Операции не возвращают значений. Тем не менее, нельзя назначить оператор `let` другой переменной, как это пытается сделать следующий код. Вы получите ошибку:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Если вы запустите эту программу, то ошибка будет выглядеть так:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

Оператор `let y = 6`не возвращает значения, так что нет ничего что можно было бы назначить переменной `x`. Такое поведение отлично от некоторых других языков, типа C и Ruby, где выражение присваивания возвращает присваиваемое значение. В таких языках можно писать код `x = y = 6` и обе переменные `x` и `y` будут иметь одинаковое значение `6`; но это не так в Rust.

Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Consider a math operation, such as `5 + 6`, which is an expression that evaluates to the value `11`. Expressions can be part of statements: in Listing 3-1, the `6` in the statement `let y = 6;` is an expression that evaluates to the value `6`. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Это выражение:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to `4`. That value gets bound to `y` as part of the `let` statement. Note that the `x + 1` line doesn’t have a semicolon at the end, unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

### Функции возвращающие значения

Функции могут возвращать значения в вызывающий их код. Мы не именуем возвращаемые значения, но мы объявляем их тип после символа (`->`). В Rust, возвращаемое значение функции является синонимом значения последнего выражения в блоке тела функции. Можно выполнить ранний возврат из функции используя ключевое слово `return` и указав значение, но большинство функций явно возвращает последнее выражение в теле. Вот пример функции возвращающей значение:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

В коде функции `five` нет вызовов функций, макросов или даже операторов  `let` - есть только одно число `5`. Это является абсолютно корректной функцией в Rust. Заметьте, что возвращаемый тип у данной функции определён как `-> i32`. Попробуйте запустить; вывод будет таким:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

Число `5` в функции `five` является возвращаемым значением функции (можно сказать что <em data-md-type="raw_html">функция `five` вычисляется в `5`</em>), вот почему возвращаемым типом является `i32`. Рассмотрим пример более детально. Есть два важных момента: первый строка `let x = five();` показывает, что мы используем значение возвращаемое функцией для инициализации переменной. Так как функция `five` возвращает `5`, то эта строка эквивалентна следующей:

```rust
let x = 5;
```

Второй момент, функция `five` не имеет входных параметров и определяет тип возвращаемого значения. Само тело функции - единственная `5` без точки с запятой. Т.к. мы хотим, чтобы функция возвращала значение, последняя строка функции должна быть выражением (не иметь после себя знак точки с запятой). В данной функции мы хотим вернуть `5` - по этому `5` должно быть выражением (не должно иметь после себя знак точки с запятой).

Рассмотрим другой пример:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Запуск кода выведет `The value of x is: 6`. Но если поместить точку с запятой в конец строки, включающей `x + 1`, то это изменит её с выражения на оператор и мы получим ошибку.

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Компиляция данного кода вызывает следующую ошибку:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

The main error message, “mismatched types,” reveals the core issue with this code. The definition of the function `plus_one` says that it will return an `i32`, but statements don’t evaluate to a value, which is expressed by `()`, the unit type. Therefore, nothing is returned, which contradicts the function definition and results in an error. In this output, Rust provides a message to possibly help rectify this issue: it suggests removing the semicolon, which would fix the error.
