# Язык программирования Rust

[The Rust Programming Language](title-page.md)

[Foreword](foreword.md)

[Introduction](ch00-00-introduction.md)

## С чего начать

- [Введение](ch01-00-getting-started.md)

    - [Установка](ch01-01-installation.md)
    - [Привет, мир!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Игра "Угадай число"](ch02-00-guessing-game-tutorial.md)

- [Общие концепции программирования](ch03-00-common-programming-concepts.md)

    - [Переменные и понятие изменяемости](ch03-01-variables-and-mutability.md)
    - [Типы данных](ch03-02-data-types.md)
    - [Как работают функции](ch03-03-how-functions-work.md)
    - [Комментарии](ch03-04-comments.md)
    - [Управление выполнением кода](ch03-05-control-flow.md)

- [Понимание владения](ch04-00-understanding-ownership.md)

    - [Что такое владение?](ch04-01-what-is-ownership.md)
    - [Ссылочные переменные и заимствование](ch04-02-references-and-borrowing.md)
    - [Срезы](ch04-03-slices.md)

- [Использование сруктуры для объединения логически связных данных](ch05-00-structs.md)

    - [Определение и инициализация структур](ch05-01-defining-structs.md)
    - [Пример использования структур](ch05-02-example-structs.md)
    - [Особености определения методов](ch05-03-method-syntax.md)

- [Перечисления и шаблоны выбора](ch06-00-enums.md)

    - [Определение перечисления](ch06-01-defining-an-enum.md)
    - [Оператор управления работой кода `match`](ch06-02-match.md)
    - [Сокращённое управление с `if let`](ch06-03-if-let.md)

## Основы Rust

- [Managing Growing Projects with Packages, Crates, and Modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)

    - [Packages and Crates](ch07-01-packages-and-crates.md)
    - [Defining Modules to Control Scope and Privacy](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Paths for Referring to an Item in the Module Tree](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Bringing Paths Into Scope with the `use` Keyword](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Separating Modules into Different Files](ch07-05-separating-modules-into-different-files.md)

- [Common Collections](ch08-00-common-collections.md)

    - [Storing Lists of Values with Vectors](ch08-01-vectors.md)
    - [Storing UTF-8 Encoded Text with Strings](ch08-02-strings.md)
    - [Storing Keys with Associated Values in Hash Maps](ch08-03-hash-maps.md)

- [Обработка ошибок](ch09-00-error-handling.md)

    - [Необрабатываемые ошибки с помощью макроса `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Обрабатываемые ошибки и `Result`](ch09-02-recoverable-errors-with-result.md)
    - [Паниковать, или нет?](ch09-03-to-panic-or-not-to-panic.md)

- [Шаблонные типы данных, типажи и время жизни](ch10-00-generics.md)

    - [Шаблонные типы данных](ch10-01-syntax.md)
    - [Типажи: определение общего поведения](ch10-02-traits.md)
    - [Проверка ссылок с помощью модификатора времени жизни](ch10-03-lifetime-syntax.md)

- [Writing Automated Tests](ch11-00-testing.md)

    - [How to Write Tests](ch11-01-writing-tests.md)
    - [Controlling How Tests Are Run](ch11-02-running-tests.md)
    - [Организация тестов](ch11-03-test-organization.md)

- [Проект Ввода/Вывода (I/O): Создание консольного приложения](ch12-00-an-io-project.md)

    - [Принятие аргументов командной строки](ch12-01-accepting-command-line-arguments.md)
    - [Чтение файлов](ch12-02-reading-a-file.md)
    - [Рефакторинг для улучшения модульности и обработки ошибок](ch12-03-improving-error-handling-and-modularity.md)
    - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
    - [Working with Environment Variables](ch12-05-working-with-environment-variables.md)
    - [Написание сообщений об ошибках в поток ошибок вместо стандартного потока вывода](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [Функциональные возможности языка Rust](ch13-00-functional-features.md)

    - [Closures: Anonymous Functions that Can Capture Their Environment](ch13-01-closures.md)
    - [Processing a Series of Items with Iterators](ch13-02-iterators.md)
    - [Improving Our I/O Project](ch13-03-improving-our-io-project.md)
    - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.md)

- [Больше о Cargo и Crates.io](ch14-00-more-about-cargo.md)

    - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md)
    - [Публикация библиотеки в Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
    - [Установка бинарных файлов из Crates.io с помощью `cargo install`](ch14-04-installing-binaries.md)
    - [Расширение Cargo пользовательскими коммандами](ch14-05-extending-cargo.md)

- [Умные указатели](ch15-00-smart-pointers.md)

    - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
    - [Treating Smart Pointers Like Regular References with the `Deref` Trait](ch15-02-deref.md)
    - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
    - [Счётчик указателей `Rc<T>`](ch15-04-rc.md)
    - [`RefCell<T>` и шабон внутренней изменяемости](ch15-05-interior-mutability.md)
    - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Многопоточность без страха](ch16-00-concurrency.md)

    - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
    - [Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.md)
    - [Shared-State Concurrency](ch16-03-shared-state.md)
    - [Расширение возможностей многопоточных программ с помощью типажей `Sync` и `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Object Oriented Programming Features of Rust](ch17-00-oop.md)

    - [Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.md)
    - [Using Trait Objects That Allow for Values of Different Types](ch17-02-trait-objects.md)
    - [Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.md)

## Расширеные темы

- [Шаблоны соответствуют структуре значений](ch18-00-patterns.md)

    - [Места использования шаблонов](ch18-01-all-the-places-for-patterns.md)
    - [Когда шаблон может не сработать](ch18-02-refutability.md)
    - [Синтаксис шаблонов](ch18-03-pattern-syntax.md)

- [Расширенные возможности](ch19-00-advanced-features.md)

    - [Небезопасный Rust](ch19-01-unsafe-rust.md)
    - [Расширенные модификаторы времени жизни](ch19-03-advanced-traits.md)
    - [Расширенные опции типажей](ch19-04-advanced-types.md)
    - [Дополнительные сведения о функциях и замыканиях](ch19-05-advanced-functions-and-closures.md)
    - [Macros](ch19-06-macros.md)

- [Заключительный проект: реализация многопоточного веб-сервера](ch20-00-final-project-a-web-server.md)

    - [Однопоточный веб-сервер](ch20-01-single-threaded.md)
    - [Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.md)
    - [Изящное завершение и освобождение ресурсов](ch20-03-graceful-shutdown-and-cleanup.md)

- [Дополнительная информация](appendix-00.md)

    - [Дополнение A: Ключевые слова](appendix-01-keywords.md)
    - [Дополнение B: Операторы](appendix-02-operators.md)
    - [C - Derivable Traits](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - Translations of the Book](appendix-06-translation.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
