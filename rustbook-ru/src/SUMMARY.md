# The Rust Programming Language

[Язык программирования Rust](title-page.md) [Предисловие](foreword.md) [Введение](ch00-00-introduction.md)

## С чего начать

- [С чего начать](ch01-00-getting-started.md)

    - [Установка](ch01-01-installation.md)
    - [Привет, Мир!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Программирование игры в загадки](ch02-00-guessing-game-tutorial.md)

- [Общие концепции программирования](ch03-00-common-programming-concepts.md)

    - [Переменные и изменяемость](ch03-01-variables-and-mutability.md)
    - [Типы Данных](ch03-02-data-types.md)
    - [Функции](ch03-03-how-functions-work.md)
    - [Комментарии](ch03-04-comments.md)
    - [Управляющие конструкции](ch03-05-control-flow.md)

- [Понимание владения](ch04-00-understanding-ownership.md)

    - [Что такое "владение"?](ch04-01-what-is-ownership.md)
    - [Ссылки и заимствование](ch04-02-references-and-borrowing.md)
    - [Тип среза](ch04-03-slices.md)

- [Использование структур для объединения связанных данных](ch05-00-structs.md)

    - [Определение и создание экземпляров структур](ch05-01-defining-structs.md)
    - [Пример программы, использующей структуры](ch05-02-example-structs.md)
    - [Синтаксис метода](ch05-03-method-syntax.md)

- [Перечисления и сопоставление с образцом](ch06-00-enums.md)

    - [Определение Enum](ch06-01-defining-an-enum.md)
    - [Конструкция потока управления `match`](ch06-02-match.md)
    - [Лаконичный поток управления с `if let`](ch06-03-if-let.md)

## Основы Rust

- [Управление растущими проектами с помощью пакетов, крейтов и модулей](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)

    - [Пакеты и крейты](ch07-01-packages-and-crates.md)
    - [Определение модулей для управления областью действия и конфиденциальностью](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Пути для ссылки на элемент в дереве модулей](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Введение путей в область видимости с помощью ключевого слова `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Separating Modules into Different Files](ch07-05-separating-modules-into-different-files.md)

- [Общие коллекции](ch08-00-common-collections.md)

    - [Хранение списков значений с векторами](ch08-01-vectors.md)
    - [Хранение закодированного текста UTF-8 со строками](ch08-02-strings.md)
    - [Хранение ключей со связанными значениями в HashMap](ch08-03-hash-maps.md)

- [Обработка ошибок](ch09-00-error-handling.md)

    - [Неустранимые ошибки с `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Устранимые ошибки с `Result`](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` или Не `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Общие типы, трейты (характеристики) и время жизни](ch10-00-generics.md)

    - [Обобщённые типы данных](ch10-01-syntax.md)
    - [Трейты (характеристики): определение разделяемого поведения](ch10-02-traits.md)
    - [Валидация ссылок посредством сроков жизни](ch10-03-lifetime-syntax.md)

- [Написание автоматических тестов](ch11-00-testing.md)

    - [Как писать тесты](ch11-01-writing-tests.md)
    - [Управление выполнением тестов](ch11-02-running-tests.md)
    - [Организация тестов](ch11-03-test-organization.md)

- [Проект с вводом-выводом: создание программы командной строки](ch12-00-an-io-project.md)

    - [Получение аргументов командной строки](ch12-01-accepting-command-line-arguments.md)
    - [Чтение файла](ch12-02-reading-a-file.md)
    - [Рефакторинг для обеспечения модульности и улучшения обработки ошибок](ch12-03-improving-error-handling-and-modularity.md)
    - [Разработка функциональности библиотеки с помощью разработки через тестирование](ch12-04-testing-the-librarys-functionality.md)
    - [Работа с переменными среды](ch12-05-working-with-environment-variables.md)
    - [Запись сообщений об ошибках в stderr вместо stdout](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Думать на Rust

- [Функциональные возможности языка: итераторы и замыкания](ch13-00-functional-features.md)

    - [Замыкания: анонимные функции, которые захватывают своё окружение](ch13-01-closures.md)
    - [Обработка последовательности элементов с помощью итераторов](ch13-02-iterators.md)
    - [Улучшение нашего проекта с вводом/выводом](ch13-03-improving-our-io-project.md)
    - [Сравнение производительности: циклы и итераторы](ch13-04-performance.md)

- [Подробнее о Cargo и Crates.io](ch14-00-more-about-cargo.md)

    - [Настройка билдов с помощью профилей выпуска](ch14-01-release-profiles.md)
    - [Публикация крейта на Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Рабочие области Cargo](ch14-03-cargo-workspaces.md)
    - [Установка бинарных файлов с Crates.io с помощью `cargo install`](ch14-04-installing-binaries.md)
    - [Расширение возможностей Cargo путём добавления пользовательских команд](ch14-05-extending-cargo.md)

- [Умные указатели](ch15-00-smart-pointers.md)

    - [Использование `Box<T>` для указания на данные в куче](ch15-01-box.md)
    - [Работа с умными указателями как с обычными ссылками с помощью трейта `Deref`](ch15-02-deref.md)
    - [Выполнение кода при очистке с помощью трейта `Drop`](ch15-03-drop.md)
    - [`Rc<T>`, умный указатель с подсчётом ссылок](ch15-04-rc.md)
    - [`RefCell<T>` и внутренняя изменяемость](ch15-05-interior-mutability.md)
    - [Ссылочные циклы могут привести к утечке памяти](ch15-06-reference-cycles.md)

- [Безбоязненный параллелизм](ch16-00-concurrency.md)

    - [Использование потоков для параллельного выполнения кода](ch16-01-threads.md)
    - [Пересылка сообщений для передачи данных между потоками](ch16-02-message-passing.md)
    - [Параллелизм с общим состоянием](ch16-03-shared-state.md)
    - [Расширяемый параллелизм с помощью трейтов `Sync` и `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Возможности объектно-ориентированного программирования Rust](ch17-00-oop.md)

    - [Характеристики объектно-ориентированных языков](ch17-01-what-is-oo.md)
    - [Использование трейт-объектов, допускающих значения разных типов](ch17-02-trait-objects.md)
    - [Реализация шаблона объектно-ориентированного проектирования](ch17-03-oo-design-patterns.md)

## Продвинутые темы

- [Шаблоны и сопоставление](ch18-00-patterns.md)

    - [Все места, где могут использоваться шаблоны](ch18-01-all-the-places-for-patterns.md)
    - [Опровержимость: может ли шаблон не соответствовать](ch18-02-refutability.md)
    - [Синтаксис шаблона](ch18-03-pattern-syntax.md)

- [Расширенные возможности](ch19-00-advanced-features.md)

    - [Небезопасный код в Rust](ch19-01-unsafe-rust.md)
    - [Продвинутые типажи](ch19-03-advanced-traits.md)
    - [Продвинутые типы](ch19-04-advanced-types.md)
    - [Продвинутые функции и замыкания](ch19-05-advanced-functions-and-closures.md)
    - [Макросы](ch19-06-macros.md)

- [Финальный проект: создание многопоточного веб-сервера](ch20-00-final-project-a-web-server.md)

    - [Создание однопоточного веб-сервера](ch20-01-single-threaded.md)
    - [Превращение нашего однопоточного сервера в многопоточный сервер](ch20-02-multithreaded.md)
    - [	Мягкое завершение работы и очистка](ch20-03-graceful-shutdown-and-cleanup.md)

- [Приложения](appendix-00.md)

    - [А — Ключевые слова](appendix-01-keywords.md)
    - [B — Операторы и символы](appendix-02-operators.md)
    - [C — Выводимые трейты](appendix-03-derivable-traits.md)
    - [D — Полезные инструменты разработки](appendix-04-useful-development-tools.md)
    - [E — Редакции](appendix-05-editions.md)
    - [F — Переводы книги](appendix-06-translation.md)
    - [G — Как создаётся Rust и «Nightly Rust»](appendix-07-nightly-rust.md)
