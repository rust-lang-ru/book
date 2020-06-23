# Язык программирования Rust

[Язык программирования Rust](title-page.md)

[Предисловие](foreword.md)

[Введение](ch00-00-introduction.md)

## С чего начать

- [Введение](ch01-00-getting-started.md)

    - [Установка](ch01-01-installation.md)
    - [Привет, мир!](ch01-02-hello-world.md)
    - [Привет, Cargo!](ch01-03-hello-cargo.md)

- [Игра "Угадай число"](ch02-00-guessing-game-tutorial.md)

- [Общие концепции программирования](ch03-00-common-programming-concepts.md)

    - [Переменные и понятие изменяемости](ch03-01-variables-and-mutability.md)
    - [Типы данных](ch03-02-data-types.md)
    - [Функции](ch03-03-how-functions-work.md)
    - [Комментарии](ch03-04-comments.md)
    - [Управление выполнением кода](ch03-05-control-flow.md)

- [Понимание владения](ch04-00-understanding-ownership.md)

    - [Что такое владение?](ch04-01-what-is-ownership.md)
    - [Ссылочные переменные и заимствование](ch04-02-references-and-borrowing.md)
    - [Срезы](ch04-03-slices.md)

- [Использование структуры для объединения логически связанных данных](ch05-00-structs.md)

    - [Определение и инициализация структур](ch05-01-defining-structs.md)
    - [Пример использования структур](ch05-02-example-structs.md)
    - [Синтаксис метода](ch05-03-method-syntax.md)

- [Перечисления и Сопоставление с образцом](ch06-00-enums.md)

    - [Определение перечисления](ch06-01-defining-an-enum.md)
    - [Оператор управления потоком выполнения `match`](ch06-02-match.md)
    - [Выразительное управление с помощью `if let`](ch06-03-if-let.md)

## Основы Rust

- [Управление растущими проектами с помощью пакетов, крейтов и модулей](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)

    - [Пакеты и крейты](ch07-01-packages-and-crates.md)
    - [Определение модулей для контроля видимости и конфиденциальности](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Пути для ссылки на элемент в дереве модуля](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Подключение путей в область видимости с помощью ключевого слова `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Разделение модулей на разные файлы](ch07-05-separating-modules-into-different-files.md)

- [Коллекции стандартной библиотеки](ch08-00-common-collections.md)

    - [Сохранение списка значений с помощью вектора](ch08-01-vectors.md)
    - [Сохранение текста с UTF-8 кодировкой в строках](ch08-02-strings.md)
    - [Хранение ключей со связанными значениями в HashMap](ch08-03-hash-maps.md)

- [Обработка ошибок](ch09-00-error-handling.md)

    - [Неустранимые ошибки с макросом `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Исправимые ошибки с `Result`](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` или не `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Обобщённые типы, типажи и время жизни](ch10-00-generics.md)

    - [Обобщённые типы данных](ch10-01-syntax.md)
    - [Типажи: определение общего поведения](ch10-02-traits.md)
    - [Проверка ссылок с временем жизни](ch10-03-lifetime-syntax.md)

- [Написание автоматизированных тестов](ch11-00-testing.md)

    - [Как писать тесты](ch11-01-writing-tests.md)
    - [Контролирование хода выполнение тестов](ch11-02-running-tests.md)
    - [Организация тестов](ch11-03-test-organization.md)

- [Проект с вводом/выводом (I/O): создание консольного приложения](ch12-00-an-io-project.md)

    - [Принятие аргументов командной строки](ch12-01-accepting-command-line-arguments.md)
    - [Чтение файла](ch12-02-reading-a-file.md)
    - [Рефакторинг для улучшения модульности и обработки ошибок](ch12-03-improving-error-handling-and-modularity.md)
    - [Развитие функциональности библиотеки разработкой на основе тестов](ch12-04-testing-the-librarys-functionality.md)
    - [Работа с переменными окружения](ch12-05-working-with-environment-variables.md)
    - [Запись сообщений ошибок в поток ошибок вместо стандартного потока вывода](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Думать на Rust

- [Функциональные возможности языка Rust: Итераторы и Замыкания](ch13-00-functional-features.md)

    - [Замыкания: анонимные функции, которые могут захватывать окружение](ch13-01-closures.md)
    - [Обработка группы элементов с помощью итераторов](ch13-02-iterators.md)
    - [Улучшение проекта ввода/вывода](ch13-03-improving-our-io-project.md)
    - [Сравнение производительности циклов и итераторов](ch13-04-performance.md)

- [Больше о Cargo и Crates.io](ch14-00-more-about-cargo.md)

    - [Настройка сборок с профилями релизов](ch14-01-release-profiles.md)
    - [Публикация библиотеки в Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Рабочие пространства Cargo](ch14-03-cargo-workspaces.md)
    - [Установка исполняемых крейтов из Crates.io командой `cargo install`](ch14-04-installing-binaries.md)
    - [Расширение Cargo пользовательскими командами](ch14-05-extending-cargo.md)

- [Умные указатели](ch15-00-smart-pointers.md)

    - [Использование `Box<T>` для ссылки на данные в куче](ch15-01-box.md)
    - [Обращение с умными указателями как с обычными ссылками с помощью `Deref` типажа](ch15-02-deref.md)
    - [Запуск кода при очистке с помощью типажа `Drop`](ch15-03-drop.md)
    - [](ch15-04-rc.md)[`Rc<T>`, умный указатель с подсчётом ссылок](ch15-04-rc.md)
    - [](ch15-05-interior-mutability.md)[`RefCell<T>` и шаблон внутренней изменяемости](ch15-05-interior-mutability.md)
    - [Ссылочные зацикливания могут приводить к утечке памяти](ch15-06-reference-cycles.md)

- [Многопоточность без страха](ch16-00-concurrency.md)

    - [Использование потоков для одновременного выполнения кода](ch16-01-threads.md)
    - [Передача данных с помощью сообщений между потоками](ch16-02-message-passing.md)
    - [Многопоточное разделяемое состояние](ch16-03-shared-state.md)
    - [Расширенная многопоточность с помощью типажей `Sync` и `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Особенности объектно-ориентированного программирования в Rust](ch17-00-oop.md)

    - [Характеристики объектно-ориентированных языков](ch17-01-what-is-oo.md)
    - [Использование типаж-объектов, допускающих значения разных типов](ch17-02-trait-objects.md)
    - [Реализация ООП шаблона проектирования](ch17-03-oo-design-patterns.md)

## Расширенные темы

- [Шаблоны и сопоставление](ch18-00-patterns.md)

    - [Все места для использования шаблонов](ch18-01-all-the-places-for-patterns.md)
    - [Возможность опровержения: может ли шаблон не совпадать](ch18-02-refutability.md)
    - [Синтаксис шаблонов](ch18-03-pattern-syntax.md)

- [Расширенные возможности](ch19-00-advanced-features.md)

    - [Небезопасный Rust](ch19-01-unsafe-rust.md)
    - [Продвинутые типажи](ch19-03-advanced-traits.md)
    - [Расширенные типы](ch19-04-advanced-types.md)
    - [Продвинутые функции и замыкания](ch19-05-advanced-functions-and-closures.md)
    - [Макросы](ch19-06-macros.md)

- [Финальный проект: создание многопоточного веб-сервера](ch20-00-final-project-a-web-server.md)

    - [Создание однопоточного веб-сервера](ch20-01-single-threaded.md)
    - [Превращение однопоточного сервера в многопоточный сервер](ch20-02-multithreaded.md)
    - [Изящное завершение и освобождение ресурсов](ch20-03-graceful-shutdown-and-cleanup.md)

- [Дополнения](appendix-00.md)

    - [Дополнение A - Ключевые слова](appendix-01-keywords.md)
    - [Дополнение Б: Операторы и обозначения](appendix-02-operators.md)
    - [Дополнение В: Выводимые типажи](appendix-03-derivable-traits.md)
    - [Дополнение Г - Средства разработки](appendix-04-useful-development-tools.md)
    - [Приложение Д - Редакции языка](appendix-05-editions.md)
    - [Приложение Е: Переводы книги](appendix-06-translation.md)
    - [Дополнение Ё - Как создаётся Rust и "Nightly Rust"](appendix-07-nightly-rust.md)
