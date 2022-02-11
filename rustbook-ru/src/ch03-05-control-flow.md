## Поток управления

The ability to run some code depending on if a condition is true, or run some code repeatedly while a condition is true, are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are `if` expressions and loops.

### Выражения `if`

Выражение `if` позволяет разделить ваш код на ветви и выполнять ту или иную ветвь кода в зависимости от условий. Вы предоставляете условие и затем пишите утверждение вида "если это условие выполняется/верное, выполнить данный блок кода; если не выполняется, не выполнять этот блок кода".

Для изучения выражения `if` создайте новый проект под названием *branches* в каталоге *projects*. В файл <em>src/main.rs</em> поместите следующий код:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Все выражения `if` начинаются с ключевого слова `if`, за которым следует условие. В данном случае условие проверяет, имеет ли переменная `number` значение меньше 5. Мы помещаем блок кода, который будет выполняться, если условие истинно, сразу после условия внутри фигурных скобок. Блоки кода, связанные с условиями в выражениях `if`, иногда называют *ветками*, так же как и ветки в выражениях `match`, которые мы обсуждали в разделе ["Сравнение догадки с секретным числом"](ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) главы 2.

Опционально можно включить выражение `else`, которое мы используем в данном примере, чтобы предоставить программе альтернативный блок выполнения кода, выполняющийся при ложном условии. Если не написать выражение `else` и условие будет ложным, то программа просто пропустит блок `if` и перейдёт к выполнению кода размещённого дальше.

Попробуйте запустить этот код. Появится следующий результат:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Попробуем поменять число `number` в значение, которое сделает условие `ложным (false)` и посмотрим, что будет:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Запустите программу снова и посмотрите на вывод:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Также стоит отметить, что условие в этом коде *должно* быть логического типа `bool`. Если условие не является `bool`, возникнет ошибка. Например, попробуйте запустить следующий код:

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

На этот раз условие `if` вычисляется в значение `3`, и Rust бросает ошибку:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Ошибка говорит, что Rust ожидал тип `bool`, но получил значение целочисленного типа. В отличии от других языков вроде Ruby и JavaScript, Rust не будет пытаться автоматически конвертировать <em>нелогические</em> типы в логические. Необходимо быть явным и всегда использовать `if` с логическим типом в качестве условия. Если нужно, чтобы блок кода `if` запускался только, когда число не равно `0`, то, например, мы можем изменить выражение `if` на следующее:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Будет напечатана следующая строка `number was something other than zero`.

#### Обработка нескольких условий с помощью `else if`

Можно использовать несколько условий, комбинируя `if` и `else` в выражении `else if`. Например:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

У этой программы есть четыре возможных пути выполнения. После её запуска вы должны увидеть следующий результат:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Во время выполнения этой программы по очереди проверяется каждое выражение `if` и выполняется первое тело, для которого условие истинно. Заметьте, что хотя 6 делится на 2, мы не видим ни вывода `number is divisible by 2`, ни текста `number is not divisible by 4, 3, or 2` из блока `else`. Так происходит потому, что Rust выполняет блок только для первого истинного условия, а обнаружив его, даже не проверяет остальные.

Использование множества выражений `else if` приводит к загромождению кода, поэтому при наличии более чем одного выражения, возможно, стоит провести рефакторинг кода. В главе 6 описана мощная конструкция ветвления Rust для таких случаев, называемая `match`.

#### Использование `if` в `let`-операторах

Поскольку `if` является выражением, его можно использовать в правой части оператора `let` для присвоения результата переменной, как в листинге 3-2.

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Листинг 3-2: Присвоение результата выражения <code>if</code> переменной</span>

Переменная `number` будет привязана к значению, которое является результатом выражения `if`. Запустим код и посмотрим, что происходит:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Вспомните, что блоки кода вычисляются последним выражением в них, а числа сами по себе также являются выражениями. В данном случае, значение всего выражения `if` зависит от того, какой блок выполняется. При этом значения, которые могут быть результатами каждого из ветвей `if`, должны быть одного типа. В Листинге 3-2, результатами обеих ветвей `if` и `else` являются целочисленный тип `i32`. Если типы не совпадают, как в следующем примере, мы получим ошибку:

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

При попытке компиляции этого кода, мы получим ошибку. Ветви `if` и `else` представляют несовместимые типы значений, и Rust точно указывает, где искать проблему в программе:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

Выражение в блоке `if` вычисляется как целочисленное, а выражение в блоке `else` вычисляется как строка. Это не сработает, потому что переменные должны иметь один тип, а Rust должен знать во время компиляции, какого типа переменная `number`. Зная тип `number`, компилятор может убедиться, что тип действителен везде, где мы используем `number`. Rust не смог бы этого сделать, если бы тип `number` определялся только во время выполнения.. Компилятор усложнился бы и давал бы меньше гарантий в отношении кода, если бы ему приходилось отслеживать несколько гипотетических типов для любой переменной.

### Повторение выполнения кода с помощью циклов

Часто бывает полезно выполнить блок кода более одного раза. Для этой задачи Rust предоставляет несколько *циклов*, которые позволяют выполнить код внутри тела цикла до конца, а затем сразу же вернуться в начало. Для экспериментов с циклами давайте создадим новый проект под названием *loops*.

В Rust есть три вида циклов: `loop`, `while` и `for`. Давайте попробуем каждый из них.

#### Повторение выполнения кода с помощью `loop`

Ключевое слово `loop` говорит Rust выполнять блок кода снова и снова до бесконечности или пока не будет явно приказано остановиться.

В качестве примера, измените код файла *src/main.rs* в каталоге проекта *loops* на код ниже:

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Когда запустим эту программу, увидим, как `again!` печатается снова и снова, пока не остановить программу вручную. Большинство терминалов поддерживают комбинацию клавиш <span class="keystroke">ctrl-c</span> для прерывания программы, которая застряла в непрерывном цикле. Попробуйте:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

Символ `^C` обозначает место, где было нажато <span class="keystroke">ctrl-c </span>. В зависимости от того, где находился код в цикле в момент получения сигнала прерывания, вы можете увидеть или не увидеть слово `again!`, напечатанное после `^C`.

К счастью, Rust также предоставляет способ выйти из цикла с помощью кода. Ключевое слово `break` нужно поместить в цикл, чтобы указать программе, когда следует прекратить выполнение цикла. Напоминаем, мы делали так в игре "Угадайка" в разделе ["Выход после правильной догадки"](ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess) главы 2, чтобы выйти из программы, когда пользователь выиграл игру, угадав правильное число.

Мы также использовали `continue` в игре "Угадайка", которая указывает программе в цикле пропустить весь оставшийся код в данной итерации цикла и перейти к следующей итерации.

Если у вас есть циклы внутри циклов, `break` и `continue` будут применяться к самому внутреннему циклу в данной точке. Дополнительно можно указать *метку цикла*, которую затем можно использовать с `break` или `continue`, чтобы обозначить, что эти ключевые слова применяются к помеченному циклу, а не к самому внутреннему циклу. Вот пример с двумя вложенными циклами:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Внешний цикл имеет метку `'counting_up`, и он будет считать от 0 до 2. Внутренняя петля без метки ведёт обратный отсчёт от 10 до 9. Первый `break`, который не содержит метку, выйдет только из внутреннего цикла. Оператор `break 'counting_up;` завершит внешний цикл. Этот код напечатает:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Возвращение значений из циклов

Одно из применений `loop` - это повторение операции, которая может закончиться неудачей, например, проверка успешности выполнения потоком своего задания. Также может понадобиться передать из цикла результат этой операции в остальную часть кода. Для этого можно добавить возвращаемое значение после выражения `break`, которое используется для остановки цикла. Это значение будет возвращено из цикла, и его можно будет использовать, как показано здесь:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Перед циклом объявляется переменная с именем `counter` и её значение инициализируется в `0`. Затем объявляется переменная с именем `result` для хранения значения, возвращаемого из цикла. На каждом проходе цикла добавляется `1` к переменной `counter` и затем проверяется, равен ли этот счётчик значению `10`. Если равен, то используется ключевое слово `break` со значением `counter * 2`. После цикла мы используем точку с запятой чтобы завершить выражение которое назначает значение переменной  `result`. В итоге печатается значение из переменной `result`, которое в данном случае равно 20.

#### Циклы с условием `while`

A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls `break`, stopping the loop. It’s possible to implement behavior like this using a combination of `loop`, `if`, `else`, and `break`; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a `while` loop. In Listing 3-3, we use `while` to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Листинг 3-3: Использование цикла <code>while</code> для выполнения кода, пока условие истинно</span>

Эта конструкция устраняет множество вложений, которые потребовались бы при использовании `loop`, `if`, `else` и `break`, и она более понятна. Пока условие истинно, код выполняется, в противном случае происходит выход из цикла.

#### Цикл по элементам коллекции с помощью `for`

Для перебора элементов коллекции, например, массива, можно использовать конструкцию `while`. Например, цикл в листинге 3-4 печатает каждый элемент массива `a`.

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Листинг 3-4: Перебор каждого элемента коллекции с помощью цикла <code>while</code></span>

Этот код выполняет перебор элементов массива. Он начинается с индекса `0`, а затем циклически выполняется, пока не достигнет последнего индекса в массиве (то есть, когда `index < 5` уже не является истиной). Выполнение этого кода напечатает каждый элемент массива:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Все пять значений массива появляются в терминале, как и ожидалось. Поскольку `index` в какой-то момент достигнет значения `5`, цикл прекратит выполнение перед попыткой извлечь шестое значение из массива.

Однако такой подход чреват ошибками. Можно вызвать панику в программе, если значение индекса или условие теста неверны. Например, если изменить определение массива `a` на четыре элемента, но забыть обновить условие на `while index < 4`, код вызовет панику. Также это медленно, поскольку компилятор добавляет код времени выполнения для обеспечения проверки нахождения индекса в границах массива на каждой итерации цикла.

В качестве более краткой альтернативы можно использовать цикл `for` и выполнять некоторый код для каждого элемента коллекции. Цикл `for` может выглядеть как код в листинге 3-5.

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Листинг 3-5: Перебор каждого элемента коллекции с помощью цикла <code>for</code></span>

When we run this code, we’ll see the same output as in Listing 3-4. More importantly, we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

При использовании цикла `for` не нужно помнить о внесении изменений в другой код, в случае изменения количества значений в массиве, как это было бы с методом, использованным в листинге 3-4.

The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a `while` loop in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that would be to use a `Range`, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

Here’s what the countdown would look like using a `for` loop and another method we’ve not yet talked about, `rev`, to reverse the range:

<span class="filename">Имя файла: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Данный код выглядит лучше, не так ли?

## Итоги

Это была обширная глава. Вы познакомились с переменными, скалярными и сложными типами данных, функциями, комментариями,  выражениями `if`  и циклами. Если хотите практиковаться с концепциями рассмотренными в данной главе, то попробуйте написать следующие программы:

- Конвертация температур между значениями по Фаренгейту к Цельсия.
- Генерирование n-го числа Фибоначчи.
- Распечатайте текст рождественской песни "Двенадцать дней Рождества", воспользовавшись повторами в песне.

Когда вы будете готовы двигаться дальше, мы поговорим о концепции в Rust, которая *не существует* обычно в других языках программирования: владение.
