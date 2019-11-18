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
    - [Ссылки и заимствование](ch04-02-references-and-borrowing.md)
    - [Срезы](ch04-03-slices.md)

- [Использование структур для объединения логически связных данных](ch05-00-structs.md)

    - [Определение и инициализация структур](ch05-01-defining-structs.md)
    - [Пример использования структур](ch05-02-example-structs.md)
    - [Особенности определения методов](ch05-03-method-syntax.md)

- [Перечисления и сопоставление с образцом](ch06-00-enums.md)

    - [Определение перечисления](ch06-01-defining-an-enum.md)
    - [Оператор `match`](ch06-02-match.md)
    - [Сокращённое управление с `if let`](ch06-03-if-let.md)

## Основы Rust

- [Управление растущими проектами с помощью пакетов, крейтов и модулей](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)

    - [Пакеты и крейты](ch07-01-packages-and-crates.md)
    - [Объявление модулей для управления областями видимости и приватностью](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Варианты указания пути на элемент в дереве модулей](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Подключение путей в область действия с помощью ключевого слова `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Разделение модулей на разные файлы](ch07-05-separating-modules-into-different-files.md)

- [Коллекции](ch08-00-common-collections.md)

    - [Сохранение списка значений с помощью вектора](ch08-01-vectors.md)
    - [Сохранение UTF-8 текста в строки](ch08-02-strings.md)
    - [Сохранение ключей с ассоциированными значениями в хэш-карты](ch08-03-hash-maps.md)

- [Обработка ошибок](ch09-00-error-handling.md)

    - [Необрабатываемые ошибки с помощью макроса `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Обрабатываемые ошибки и `Result`](ch09-02-recoverable-errors-with-result.md)
    - [Паниковать, или нет?](ch09-03-to-panic-or-not-to-panic.md)

- [Шаблонные типы данных, типажи и время жизни](ch10-00-generics.md)

    - [Шаблонные типы данных](ch10-01-syntax.md)
    - [Типажи: определение общего поведения](ch10-02-traits.md)
    - [Проверка ссылок с помощью времён жизни](ch10-03-lifetime-syntax.md)

- [Написание автоматизированных тестов](ch11-00-testing.md)

    - [Как писать тесты](ch11-01-writing-tests.md)
    - [Контроль работы тестов](ch11-02-running-tests.md)
    - [Организация тестов](ch11-03-test-organization.md)

- [Проект Ввода/Вывода (I/O): Создание консольного приложения](ch12-00-an-io-project.md)

    - [Получение аргументов командной строки](ch12-01-accepting-command-line-arguments.md)
    - [Чтение файлов](ch12-02-reading-a-file.md)
    - [Рефакторинг для улучшения модульности и обработки ошибок](ch12-03-improving-error-handling-and-modularity.md)
    - [Разработка функциональности библиотеки при помощи TDD](ch12-04-testing-the-librarys-functionality.md)
    - [Работа с переменными окружения](ch12-05-working-with-environment-variables.md)
    - [Написание сообщений об ошибках в поток ошибок вместо стандартного потока вывода](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Думать на Rust

- [Функциональные возможности языка Rust: итераторы и замыкания](ch13-00-functional-features.md)

    - [Замыкания: анонимные функции, которые могут захватывать собственное окружение](ch13-01-closures.md)
    - [Обработка группы элементов с помощью итераторов](ch13-02-iterators.md)
    - [Улучшение нашего I/O проекта](ch13-03-improving-our-io-project.md)
    - [Сравнение производительности: циклы vs итераторы](ch13-04-performance.md)

- [Больше о Cargo и Crates.io](ch14-00-more-about-cargo.md)

    - [Выборочная сборка с профилями выпуска](ch14-01-release-profiles.md)
    - [Публикация библиотеки в Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Рабочие области Cargo](ch14-03-cargo-workspaces.md)
    - [Установка бинарных файлов из Crates.io с помощью `cargo install`](ch14-04-installing-binaries.md)
    - [Расширение Cargo пользовательскими командами](ch14-05-extending-cargo.md)

- [Умные указатели](ch15-00-smart-pointers.md)

    - [Использование `Box<T>` для доступа к данным в куче](ch15-01-box.md)
    - [Работа с умным указателем, как с обычной ссылкой, с помощью типажа `Deref`](ch15-02-deref.md)
    - [Запуск кода при очистке с типажом `Drop`](ch15-03-drop.md)
    - [Счётчик указателей `Rc<T>`](ch15-04-rc.md)
    - [`RefCell<T>` и шаблон внутренней изменяемости](ch15-05-interior-mutability.md)
    - [Циклические ссылки и утечки памяти](ch15-06-reference-cycles.md)

- [Многопоточность без страха](ch16-00-concurrency.md)

    - [Использование потоков для параллельного запуска кода](ch16-01-threads.md)
    - [Отправка сообщений между потоками для передачи данных](ch16-02-message-passing.md)
    - [Многопоточность с общим состоянием](ch16-03-shared-state.md)
    - [Расширение возможностей многопоточных программ с помощью типажей `Sync` и `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [ООП возможности в Rust](ch17-00-oop.md)

    - [Характеристики ООП](ch17-01-what-is-oo.md)
    - [Использование трейт-объектов для получения значений разных типов](ch17-02-trait-objects.md)
    - [Реализация одного из паттернов ООП](ch17-03-oo-design-patterns.md)

## Расширенные темы

- [Шаблоны и сопоставление](ch18-00-patterns.md)

    - [Места использования шаблонов](ch18-01-all-the-places-for-patterns.md)
    - [Когда шаблон может не сработать](ch18-02-refutability.md)
    - [Синтаксис шаблонов](ch18-03-pattern-syntax.md)

- [Расширенные возможности](ch19-00-advanced-features.md)

    - [Небезопасный Rust](ch19-01-unsafe-rust.md)
    - [Расширенные опции типажей](ch19-03-advanced-traits.md)
    - [Расширенные типы](ch19-04-advanced-types.md)
    - [Дополнительные сведения о функциях и замыканиях](ch19-05-advanced-functions-and-closures.md)
    - [Макросы](ch19-06-macros.md)

- [Заключительный проект: реализация многопоточного веб-сервера](ch20-00-final-project-a-web-server.md)

    - [Однопоточный веб-сервер](ch20-01-single-threaded.md)
    - [Делаем наш однопоточный сервер многопоточным](ch20-02-multithreaded.md)
    - [Изящное завершение и освобождение ресурсов](ch20-03-graceful-shutdown-and-cleanup.md)

- [Дополнения](appendix-00.md)

    - [Дополнение A - Ключевые слова](appendix-01-keywords.md)
    - [Дополнение Б - Операторы](appendix-02-operators.md)
    - [Дополнение В - Выводимые типажи](appendix-03-derivable-traits.md)
    - [Дополнение Г - Полезные инструменты разработки](appendix-04-useful-development-tools.md)
    - [Дополнение Д - Редакции](appendix-05-editions.md)
    - [Дополнение Е - Переводы книги](appendix-06-translation.md)
    - [Дополнение Ё - Как создаётся Rust и "Nightly Rust"](appendix-07-nightly-rust.md)
