# Программируем игру Угадайка

Let’s jump into Rust by working through a hands-on project together! This chapter introduces you to a few common Rust concepts by showing you how to use them in a real program. You’ll learn about `let`, `match`, methods, associated functions, using external crates, and more! In the following chapters, we’ll explore these ideas in more detail. In this chapter, you’ll practice the fundamentals.

Мы реализуем классическую программу для начинающих: угадывание числа. Определим как она будет работать. Программа генерирует случайное целое число от 1 до 100. Затем она предлагает игроку ввести и отгадать число. Если оно больше или меньше предложенного игроком, то  программа сообщит об этом. Если игрок угадал число, то программа выведет поздравление и завершится.

## Настройка нового проекта

Для настройки нового проекта перейдите в каталог *projects*, который вы создали в главе 1, и создайте новый проект с использованием Cargo, как показано ниже:

```console
$ cargo new guessing_game
$ cd guessing_game
```

Первая команда, `cargo new`, принимает в качестве первого аргумента имя проекта (`guessing_game`). Вторая команда изменяет каталог на новый каталог проекта.

Загляните в созданный файл *Cargo.toml*:

<span class="filename">Имя файла: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Как вы уже видели в Главе 1, `cargo new` создаёт программу "Hello, world!". Посмотрите файл *src/main.rs*:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Теперь давайте скомпилируем программу "Hello, world!" и сразу запустим её, используя команду `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Команда `run` пригодится, когда необходимо ускоренно выполнить итерацию проекта, мы так собираемся сделать в этом проекте, быстро тестируя каждую итерацию, прежде чем перейти к следующей.

Снова откройте файл *src/main.rs*. Весь код вы будете писать в этом файле.

## Обработка отгадки

Первая часть программы игры в угадывание будет запрашивать ввод у пользователя, обрабатывать значение ввода и проверять, находится ли значение в ожидаемой форме. Для начала мы позволим игроку ввести его предположение. Введите код из Листинга 2-1 в *src/main.rs*.

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Listing 2-1: Code that gets a guess from the user and prints it</span>

Этот код содержит много информации, поэтому давайте рассмотрим его построчно. Чтобы получить пользовательский ввод и затем вывести результат в качестве вывода, нам нужно включить в область видимости библиотеку ввода/вывода `io`. Библиотека `io` является частью стандартной библиотеки, известной как `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

По умолчанию Rust имеет несколько элементов, заданных в стандартной библиотеке, которые он включает в область видимости каждой программы. Этот набор называется *прелюдией (prelude)*, и [в документации по стандартной библиотеке](../std/prelude/index.html) можно увидеть все входящее в ее состав элементы.

Если тип, который требуется использовать, отсутствует в прелюдии, его нужно явно ввести в область видимости с помощью оператора `use`. Использование библиотеки `std::io` предоставляет ряд полезных функциональных возможностей, включая способность принимать пользовательский ввод.

Как уже отмечалось в главе 1, функция `main` является точкой входа в программу:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

Синтаксис `fn` объявляет новую функцию, круглые скобки `()` указывают на отсутствие параметров, а фигурная скобка `{` обозначает начало тела функции.

Также в главе 1 упоминалось, что `println!` - это макрос, который печатает строку на экран:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Этот код печатает подсказку об игре и запрашивает пользовательский ввод.

### Хранение значений с помощью переменных

Next, we’ll create a *variable* to store the user input, like this:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Now the program is getting interesting! There’s a lot going on in this little line. We use the `let` statement to create the variable. Here’s another example:

```rust,ignore
let apples = 5;
```

This line creates a new variable named `apples` and binds it to the value 5. In Rust, variables are immutable by default. We’ll be discussing this concept in detail in the [“Variables and Mutability”](ch03-01-variables-and-mutability.html#variables-and-mutability)<!-- ignore --> section in Chapter 3. To make a variable mutable, we add `mut` before the variable name:

```rust,ignore
let apples = 5; // неизменяемая
let mut bananas = 5; // изменяемая
```

> Примечание: Синтаксис `//` означает начало комментария, который продолжается до конца строки. Rust игнорирует все содержимое комментариев. Подробнее о комментариях мы поговорим в [главе 3](ch03-04-comments.html).

Возвращаясь к программе игры Угадайка, теперь вы знаете, что `let mut guess` предоставит изменяемую переменную с именем `guess`. Знак равенства (`=`) сообщает Rust, что сейчас нужно связать что-то с этой переменной. Справа от знака равенства находится значение, связанное с `guess`, которое является результатом вызова функции `String::new`, возвращающей новый экземпляр `String`. <a href="../std/string/struct.String.html" data-md-type="link">`String`</a> - это тип строки, предоставляемый стандартной библиотекой, который является расширяемым фрагментом текста в кодировке UTF-8.

The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. An *associated function* is a function that’s implemented on a type, in this case `String`. This `new` function creates a new, empty string. You’ll find a `new` function on many types, because it’s a common name for a function that makes a new value of some kind.

В целом, строка `let mut guess = String::new();` создала изменяемую переменную, которая связывается с новым, пустым экземпляром `String`. Фух!

### Получение пользовательского ввода

Напомним, мы подключили функциональность ввода/вывода из стандартной библиотеки с помощью `use std::io;` в первой строке программы. Теперь мы вызовем функцию `stdin` из модуля `io`, которая позволит нам обрабатывать пользовательский ввод:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Если бы мы не импортировали библиотеку `io` с помощью `use std::io` в начале программы, мы все равно могли бы использовать эту функцию, записав вызов этой функции как `std::io::stdin`. Функция `stdin` возвращает экземпляр [`std::io::Stdin`](../std/io/struct.Stdin.html), который является типом, представляющим дескриптор стандартного ввода для вашего терминала.

Next, the line `.read_line(&mut guess)` calls the [`read_line`](../std/io/struct.Stdin.html#method.read_line)<!-- ignore --> method on the standard input handle to get input from the user. We’re also passing `&mut guess` as the argument to `read_line` to tell it what string to store the user input in. The full job of `read_line` is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

Символ `&` указывает, что этот аргумент является *ссылкой*, который предоставляет возможность нескольким частям вашего кода получить доступ к одному фрагменту данных без необходимости копировать эти данные в память несколько раз. Ссылки - это сложная функциональная возможность, а одним из главных преимуществ Rust является безопасность и простота использования ссылок. Чтобы дописать эту программу, вам не понадобится знать много таких подробностей. Пока вам достаточно знать, что ссылки, как и переменные, по умолчанию неизменяемы. Соответственно, чтобы сделать её изменяемой, нужно написать `&mut guess`, а не `&guess`. (В главе 4 ссылки будут описаны более подробно).

### Обработка потенциального сбоя с помощью типа `Result`

Мы все ещё продолжаем работать над выражением начатым с <code>io::stdin</code>. Хотя сейчас мы обсуждаем уже третью строку, эта строка по-прежнему является одной логической частью всего выражения. Следующая часть выражения, третья строка, метод:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Мы могли бы написать этот код так:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Однако одну длинную строку трудно читать, поэтому лучше разделить ее. При вызове метода с помощью синтаксиса `.method_name()` часто целесообразно вводить новую строку и другие пробельные символы, чтобы разбить длинные строки. Теперь давайте обсудим, что делает эта строка.

As mentioned earlier, `read_line` puts whatever the user enters into the string we pass to it, but it also returns a value—in this case, an [`io::Result`](../std/io/type.Result.html)<!-- ignore -->. Rust has a number of types named `Result` in its standard library: a generic <a href="../std/result/enum.Result.html" data-md-type="link">`Result`</a><!-- ignore --> as well as specific versions for submodules, such as `io::Result`. The `Result` types are [*enumerations*](ch06-00-enums.html)<!-- ignore -->, often referred to as *enums*, which can have a fixed set of possibilities known as *variants*. Enums are often used with `match`, a conditional that makes it convenient to execute different code based on which variant an enum value is when the conditional is evaluated.

В главе 6 перечисления будут рассмотрены более подробно. Назначение всех типов `Result` заключается в передаче информации для обработки ошибок.

Вариантами `Result` являются `Ok` и `Err`. Вариант `Ok` указывает, что операция завершилась успешно, а внутри `Ok` находится успешно сгенерированное значение. Вариант `Err` означает, что операция не удалась, а `Err` содержит информацию о причинах неудачи.

Values of the `Result` type, like values of any type, have methods defined on them. An instance of `io::Result` has an [`expect` method](../std/result/enum.Result.html#method.expect)<!-- ignore --> that you can call. If this instance of `io::Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`. If the `read_line` method returns an `Err`, it would likely be the result of an error coming from the underlying operating system. If this instance of `io::Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

Если не вызвать `expect`, программа скомпилируется, но будет получено предупреждение:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust предупреждает о не использовании значения `Result`, возвращаемого из `read_line`, показывая, что программа не учла возможность возникновения ошибки.

Правильный способ убрать предупреждение - это написать обработку ошибок, но в нашем случае мы просто хотим аварийно завершить программу при возникновении проблемы, поэтому используем `expect`. О способах восстановления после ошибок вы узнаете в [главе 9](ch09-02-recoverable-errors-with-result.html).

### Напечатать значений с помощью заполнителей `println!`

Aside from the closing curly bracket, there’s only one more line to discuss in the code so far:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Эта строка печатает строку, которая теперь содержит ввод пользователя. Набор фигурных скобок `{}` является заполнителем: думайте о `{}` как о маленьких крабовых клешнях, удерживающих значение на месте. С помощью фигурных скобок можно вывести более одного значения: первый набор фигурных скобок содержит первое значение, указанное после форматирующей строки, второй набор - второе значение и так далее. Печать нескольких значений за один вызов `println!` будет выглядеть следующим образом:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

Этот код напечатает `x = 5 and y = 10`.

### Тестирование первой части

Давайте протестирует первую часть игры. Запустите её используя `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

На данном этапе первая часть игры завершена: мы получаем ввод с клавиатуры и затем печатаем его.

## Генерация секретного числа

Далее нам нужно сгенерировать секретное число, которое пользователь попытается угадать. Секретное число должно быть каждый раз разным, чтобы в игру можно было играть несколько раз. Мы будем использовать случайное число в диапазоне от 1 до 100, чтобы игра не была слишком сложной. Rust пока не включает функциональность случайных чисел в свою стандартную библиотеку. Однако команда Rust предоставляет [`rand` crate](https://crates.io/crates/rand) с подобной функциональностью.

### Использование пакета для получения дополнительной функциональности

Помните, что пакет (crate) - это коллекция файлов исходного кода Rust. Проект, создаваемый нами, представляет собой <br> *бинарный пакет (binary crate)*, который является исполняемым файлом. Пакет `rand` - это *библиотечный пакет (library crate)*, содержащий код, который предназначен для использования в других программах и поэтому не может исполняться сам по себе.

Координация работы внешних пакетов является тем местом, где Cargo действительно блистает. Чтобы начать писать код, использующий `rand`, необходимо изменить файл *Cargo.toml*, включив в него в качестве зависимости пакет `rand`. Итак, откройте этот файл и добавьте следующую строку внизу под заголовком секции `[dependencies]`, созданным для вас Cargo. Обязательно укажите `rand` в точности как здесь, с таким же номером версии, иначе примеры кода из этого урока могут не заработать.

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Имя файла: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

In the *Cargo.toml* file, everything that follows a header is part of that section that continues until another section starts. In `[dependencies]` you tell Cargo which external crates your project depends on and which versions of those crates you require. In this case, we specify the `rand` crate with the semantic version specifier `0.8.3`. Cargo understands [Semantic Versioning](http://semver.org)<!-- ignore --> (sometimes called *SemVer*), which is a standard for writing version numbers. The number `0.8.3` is actually shorthand for `^0.8.3`, which means any version that is at least `0.8.3` but below `0.9.0`. Cargo considers these versions to have public APIs compatible with version `0.8.3`, and this specification ensures you’ll get the latest patch release that will still compile with the code in this chapter. Any version `0.9.0` or greater is not guaranteed to have the same API as what the following examples use.

Now, without changing any of the code, let’s build the project, as shown in Listing 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Листинг 2-2: Результат выполнения <code>cargo build</code> после добавления пакета rand в качестве зависимости</span>

Вы можете увидеть другие номера версий (но все они будут совместимы с кодом, благодаря SemVer!), другие строки (в зависимости от операционной системы), а также строки могут быть расположены в другом порядке.

Когда мы включаем внешнюю зависимость, Cargo берет последние версии всего, что нужно этой зависимости, из *реестра (registry)*, который является копией данных с [Crates.io](https://crates.io/). Crates.io - это место, где участники экосистемы Rust размещают свои проекты Rust с открытым исходным кодом для использования другими.

After updating the registry, Cargo checks the `[dependencies]` section and downloads any crates listed that aren’t already downloaded. In this case, although we only listed `rand` as a dependency, Cargo also grabbed other crates that `rand` depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you won’t get any output aside from the `Finished` line. Cargo knows it has already downloaded and compiled the dependencies, and you haven’t changed anything about them in your *Cargo.toml* file. Cargo also knows that you haven’t changed anything about your code, so it doesn’t recompile that either. With nothing to do, it simply exits.

Если открыть файл *src/main.rs*, внести незначительные изменения, а затем сохранить его и снова произвести сборку, то вы увидите только две строки вывода:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Эти строки показывают, что Cargo обновляет сборку только на основании вашего крошечного изменения в файле *src/main.rs*. Поскольку зависимости не изменились, Cargo знает, что может повторно использовать ранее загруженные и скомпилированные зависимости.

#### Обеспечение воспроизводимых сборок с помощью файла *Cargo.lock*

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.4 of the `rand` crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the *Cargo.lock* file the first time you run `cargo build`, so we now have this in the *guessing_game* directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the *Cargo.lock* file. When you build your project in the future, Cargo will see that the *Cargo.lock* file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at `0.8.3` until you explicitly upgrade, thanks to the *Cargo.lock* file.

#### Обновление пакета для получения новой версии

Если вы *захотите* обновить пакет, Cargo предоставляет команду `update`, которая игнорирует файл *Cargo.lock* и определяет последние версии, соответствующие вашим спецификациям из файла *Cargo.toml*. После этого Cargo запишет эти версии в файл *Cargo.lock*. Иначе, по умолчанию, Cargo будет искать только версии больше `0.8.3` , но при этом меньше `0.9.0`. Если пакет `rand` имеет две новые версии `0.8.4` и `0.9.0`, то при запуске `cargo update` вы увидите следующее:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Cargo игнорирует релиз `0.9.0`. В этот момент также появится изменение в файле *Cargo.lock*, указывающее на то, что версия `rand`, которая теперь используется, равна `0.8.4`. Чтобы использовать `rand` версии `0.9.0` или любой другой версии из серии `0.9.x`, необходимо обновить файл *Cargo.toml* следующим образом:

```toml
[dependencies]
rand = "0.9.0"
```

В следующий раз, при запуске `cargo build`, Cargo обновит реестр доступных пакетов и пересмотрит ваши требования к `rand` в соответствии с новой версией, которую вы указали.

There’s a lot more to say about [Cargo](http://doc.crates.io)<!-- ignore --> and [its ecosystem](http://doc.crates.io/crates-io.html)<!-- ignore --> which we’ll discuss in Chapter 14, but for now, that’s all you need to know. Cargo makes it very easy to reuse libraries, so Rustaceans are able to write smaller projects that are assembled from a number of packages.

### Генерация случайного числа

Let’s start using `rand` to generate a number to guess. The next step is to update *src/main.rs*, as shown in Listing 2-3.

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Листинг 2-3: Добавление кода для генерации случайного числа</span>

Сначала мы добавляем строку `use rand::Rng`. Типаж `Rng` определяет методы, реализующие генераторы случайных чисел, и этот типаж должен быть в области видимости, чтобы можно было использовать эти методы. В главе 10 мы подробно рассмотрим типажи.

Далее мы добавляем две строки посередине. В первой строке вызов функции `rand::thread_rng`, предоставляющей нам специальный генератор случайных чисел, который мы собираемся использовать: локальный для текущего потока выполнения и заполняемый операционной системой. Затем вызываем метод `gen_range` на генераторе случайных чисел. Этот метод определяется типажом `Rng`, который мы ввели в область видимости с помощью оператора `use rand::Rng`. Метод `gen_range` принимает выражение диапазона в качестве аргумента и генерирует случайное число в пределах диапазона. Выражение диапазона, которое здесь используется, имеет форму `start...end` и является инклюзивным по нижней границе, но эксклюзивным по верхней, поэтому нужно указать `1...101`, чтобы запросить число от 1 до 100. Как вариант, можно передать диапазон `1..=100`, что будет эквивалентно.

> Note: You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the `cargo doc --open` command will build documentation provided by all of your dependencies locally and open it in your browser. If you’re interested in other functionality in the `rand` crate, for example, run `cargo doc --open` and click `rand` in the sidebar on the left.

Во второй новой строке печатается секретный номер. Полезно, пока разрабатывается программа, иметь возможность тестировать ее, но в финальной версии мы это удалим. Конечно это не похоже на игру, если программа печатает ответ сразу после запуска!

Попробуйте запустить программу несколько раз:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Вы должны получить разные случайные числа, и все они должны быть числами в диапазоне от 1 до 100. Отличная работа!

## Сравнение догадки с секретным числом

Now that we have user input and a random number, we can compare them. That step is shown in Listing 2-4. Note that this code won’t compile quite yet, as we will explain.

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Листинг 2-4: Обработка возможных возвращаемых значений при сравнении двух чисел</span>

First we add another `use` statement, bringing a type called `std::cmp::Ordering` into scope from the standard library. The `Ordering` type is another enum and has the variants `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.

Then we add five new lines at the bottom that use the `Ordering` type. The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the `guess` to the `secret_number`. Then it returns a variant of the `Ordering` enum we brought into scope with the `use` statement. We use a [`match`](ch06-02-match.html)<!-- ignore --> expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

Выражение `match` состоит из *веток (arms)*. Ветка состоит из *шаблона* для сопоставления и кода, который будет запущен, если значение, переданное в `match`, соответствует шаблону этой ветки. Rust принимает значение, заданное `match`, и по очереди просматривает шаблон каждой ветки. Шаблоны и конструкция `match` - это мощные возможности Rust, позволяющие выразить множество ситуаций, с которыми может столкнуться ваш код, и гарантировать их обработку. Эти возможности будут подробно раскрыты в Главе 6 и Главе 18 соответственно.

Let’s walk through an example with the `match` expression we use here. Say that the user has guessed 50 and the randomly generated secret number this time is 38. When the code compares 50 to 38, the `cmp` method will return `Ordering::Greater`, because 50 is greater than 38. The `match` expression gets the `Ordering::Greater` value and starts checking each arm’s pattern. It looks at the first arm’s pattern, `Ordering::Less`, and sees that the value `Ordering::Greater` does not match `Ordering::Less`, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is `Ordering::Greater`, which *does* match `Ordering::Greater`! The associated code in that arm will execute and print `Too big!` to the screen. The `match` expression ends because it has no need to look at the last arm in this scenario.

Однако, код в листинге 2-4 все ещё не скомпилируется. Давайте попробуем:

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Суть ошибки заключается в наличии *несовпадающих типов*. У Rust строгая, статическая система типов. Однако он также имеет вывод типов. Когда мы написали `let mut guess = String::new()`, Rust смог сделать вывод, что `guess` должна быть `String` и не заставил указывать тип. С другой стороны, `secret_number` - это числовой тип. Несколько типов чисел в Rust могут иметь значение от 1 до 100: `i32`, 32-битное число; `u32`, беззнаковое 32-битное число; `i64`, 64-битное число, а также другие. Если не указано иное, Rust по умолчанию использует `i32`, который будет типом `secret_number`, если не добавлять информацию о типе в другом месте, которая заставит Rust вывести другой числовой тип. Причина ошибки заключается в том, что Rust не может сравнить строку и числовой тип.

В конечном итоге, необходимо преобразовать `String`, считываемую программой в качестве входных данных, в реальный числовой тип, чтобы иметь возможность числового сравнения с секретным числом. Для этого добавьте эту строку в тело функции `main`:

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Вот эта строка:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

We create a variable named `guess`. But wait, doesn’t the program already have a variable named `guess`? It does, but helpfully Rust allows us to *shadow* the previous value of `guess` with a new one. Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess` for example. We’ll cover this in more detail in Chapter 3, but for now know that this feature is often used when you want to convert a value from one type to another type.

Мы связываем эту новую переменную с выражением `guess.trim().parse()`. Переменная `guess` в этом выражении относится к исходной переменной `guess`, которая содержала входные данные в виде строки. Метод `trim` на экземпляре `String` удалит любые пробельные символы в начале и конце строки для того, чтобы мы могли сопоставить строку с `u32`, которая содержит только числовые данные. Пользователь должен нажать <span class="keystroke">enter</span>, чтобы выполнить `read_line` и ввести свою догадку, при этом в строку добавится символ новой строки. Например, если пользователь набирает <span class="keystroke">5</span> и нажимает <span class="keystroke">enter</span>, `guess` будет выглядеть так: `5\n`. Символ `\n` означает "новая строка". (В Windows нажатие <span class="keystroke">enter</span> сопровождается возвратом каретки и новой строкой, `\r\n`). Метод `trim` убирает `\n` или `\r\n`, оставляя только `5`.

[Метод строк `parse`](../std/primitive.str.html#method.parse) преобразует строку в некоторое число. Поскольку этот метод может преобразовывать различные числовые типы, мы должны сообщить Rust конкретный числовой тип, который нам нужен, используя `let guess: u32`. Двоеточие (`:`) после `guess` сообщает Rust, что мы будем аннотировать тип переменной. В Rust есть несколько встроенных числовых типов. Представленный здесь тип `u32` - это беззнаковое 32-битное целое число. Это хороший выбор по умолчанию для небольшого положительного числа. О других числовых типах вы узнаете в Главе 3. Кроме того, аннотация `u32` в этом примере программы и сравнение с `secret_number` позволяет Rust сделать вывод, что `secret_number` также должен быть <code>u32</code>. Таким образом, теперь сравнение будет проводиться между двумя значениями одного типа!

The `parse` method will only work on characters that can logically be converted into numbers and so can easily cause errors. If, for example, the string contained `A👍%`, there would be no way to convert that to a number. Because it might fail, the `parse` method returns a `Result` type, much as the `read_line` method does (discussed earlier in <a href="#handling-potential-failure-with-the-result-type" data-md-type="link">“Handling Potential Failure with the `Result` Type”</a><!-- ignore -->). We’ll treat this `Result` the same way by using the `expect` method again. If `parse` returns an `Err` `Result` variant because it couldn’t create a number from the string, the `expect` call will crash the game and print the message we give it. If `parse` can successfully convert the string to a number, it will return the `Ok` variant of `Result`, and `expect` will return the number that we want from the `Ok` value.

Давайте запустим программу теперь!

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Хорошо! Несмотря на то, что были добавлены пробелы перед догадкой 76, программа все равно вывела пользовательскую догадку 76. Запустите программу несколько раз, чтобы проверить разное поведение при различных типах ввода: задайте число правильно, задайте слишком большое число и задайте слишком маленькое число.

Сейчас у нас работает большая часть игры, но пользователь может сделать только одну догадку. Давайте изменим это, добавив цикл!

## Возможность нескольких догадок с помощью циклов

The `loop` keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number:

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

As you can see, we’ve moved everything from the guess input prompt onward into a loop. Be sure to indent the lines inside the loop another four spaces each and run the program again. The program will now ask for another guess forever, which actually introduces a new problem. It doesn’t seem like the user can quit!

The user could always interrupt the program by using the keyboard shortcut <span class="keystroke">ctrl-c</span>. But there’s another way to escape this insatiable monster, as mentioned in the `parse` discussion in [“Comparing the Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!-- ignore -->: if the user enters a non-number answer, the program will crash. We can take advantage of that to allow the user to quit, as shown here:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Typing `quit` will quit the game, but as you’ll notice so will entering any other non-number input. This is suboptimal to say the least; we want the game to also stop when the correct number is guessed.

### Выход после правильной догадки

Давайте запрограммируем игру на выход при выигрыше пользователя, добавив оператор `break`:

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Добавление строки `break` после `You win!` заставляет программу выйти из цикла, когда пользователь правильно угадает секретное число. Выход из цикла также означает выход из программы, так как цикл является последней частью `main`.

### Обработка недопустимого ввода

Для дальнейшего улучшения поведения игры вместо аварийного завершения программы при вводе пользователем не числовых значений, давайте заставим игру игнорировать не числовые символы, так пользователь сможет продолжать пытаться угадать верное число. Мы можем сделать это, изменив строку, где `guess` преобразуется из `String` в `u32`, как показано в листинге 2-5.

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Листинг 2-5. Игнорирование нечисловой догадки и запрос другой догадки вместо завершения программы</span>

Мы переключаем вызов `expect` на выражение `match`, чтобы перейти от аварийного завершения при ошибке к обработке ошибки. Помните, что `parse` возвращает тип `Result`, а `Result` - это перечисление, которое имеет варианты `Ok` и `Err`. Здесь мы используем выражение `match`, как и в случае с результатом `Ordering` метода `cmp`.

If `parse` is able to successfully turn the string into a number, it will return an `Ok` value that contains the resulting number. That `Ok` value will match the first arm’s pattern, and the `match` expression will just return the `num` value that `parse` produced and put inside the `Ok` value. That number will end up right where we want it in the new `guess` variable we’re creating.

Если метод `parse` *не способен* превратить строку в число, он вернёт значение `Err`, которое содержит более подробную информацию об ошибке. Значение `Err` не совпадает с шаблоном `Ok(num)` в первой ветке `match`, но совпадает с шаблоном `Err(_)` второй ветки. Подчёркивание `_` является всеохватывающим выражением. В этой ветке мы говорим, что хотим обработать совпадение всех значений `Err`, независимо от того, какая информация находится внутри `Err`. Таким образом, в случае неспособности получить число, программа будет выполнять код второй ветки <code>match</code>, который состоит из выражения `continue`, которое выполняет переход программы на следующую итерацию цикла `loop`. В итоге программа игнорирует все ошибки метода `parse`, которые могут встретится!

Все в программе теперь должно работать как положено. Давайте попробуем:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Awesome! With one tiny final tweak, we will finish the guessing game. Recall that the program is still printing the secret number. That worked well for testing, but it ruins the game. Let’s delete the `println!` that outputs the secret number. Listing 2-6 shows the final code.

<span class="filename">Имя файла: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Листинг 2-6: Полный код игры угадывания числа</span>

## Итоги

На данный момент вы успешно создали игру угадайку. Поздравляем!

Этот проект - практический способ познакомить вас со многими новыми концепциями Rust: `let`, `match`, функции, использование внешних пакетов и многое другое. В следующих нескольких главах вы изучите эти концепции более подробно. Глава 3 охватывает понятия, которые есть в большинстве языков программирования, такие как переменные, типы данных и функции, и показывает, как использовать их в Rust. В главе 4 рассматривается владение, особенность, которая отличает Rust от других языков. В главе 5 обсуждаются структуры и синтаксис методов, а в главе 6 объясняется, как работают перечисления.
