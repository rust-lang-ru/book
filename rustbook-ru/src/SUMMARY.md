# Язык программирования Rust

## С чего начать

- [Введение](ch01-00-introduction.md)
    - [Установка](ch01-01-installation.md)
    - [Привет, мир!](ch01-02-hello-world.md)

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

- [Модули](ch07-00-modules.md)
    - [`mod` и файловая система](ch07-01-mod-and-the-filesystem.md)
    - [Управление доступом с помощью ключевого слова `pub`](ch07-02-controlling-visibility-with-pub.md)
    - [Импорт имен с `use`](ch07-03-importing-names-with-use.md)

- [Коллекции стандартной библиотеки](ch08-00-common-collections.md)
    - [Векторы](ch08-01-vectors.md)
    - [Строки](ch08-02-strings.md)
    - [Хеш-карты](ch08-03-hash-maps.md)

- [Обработка ошибок](ch09-00-error-handling.md)
    - [Необрабатываемые ошибки с помощью макроса `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Обрабатываемые ошибки и `Result`](ch09-02-recoverable-errors-with-result.md)
    - [Паниковать, или нет?](ch09-03-to-panic-or-not-to-panic.md)

- [Шаблонные типы данных, типажи и время жизни](ch10-00-generics.md)
    - [Шаблонные типы данных](ch10-01-syntax.md)
    - [Типажи: определение общего поведения](ch10-02-traits.md)
    - [Проверка ссылок с помощью модификатора времени жизни](ch10-03-lifetime-syntax.md)

- [Тестирование](ch11-00-testing.md)
    - [Написание тестов](ch11-01-writing-tests.md)
    - [Запуск тестов](ch11-02-running-tests.md)
    - [Организация тестов](ch11-03-test-organization.md)

- [Проект Ввода/Вывода (I/O): Создание консольного приложения](ch12-00-an-io-project.md)
    - [Принятие аргументов командной строки](ch12-01-accepting-command-line-arguments.md)
    - [Чтение файлов](ch12-02-reading-a-file.md)
    - [Рефакторинг для улучшения модульности и обработки ошибок](ch12-03-improving-error-handling-and-modularity.md)
    - [Разработка функционала библиотеки с "разработкой через тестирование"](ch12-04-testing-the-librarys-functionality.md)
    - [Работа с системными переменными](ch12-05-working-with-environment-variables.md)
    - [Написание сообщений об ошибках в поток ошибок вместо стандартного потока вывода](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Думать на языке Rust

- [Функциональные возможности языка Rust](ch13-00-functional-features.md)
    - [Замыкания](ch13-01-closures.md)
    - [Итераторы](ch13-02-iterators.md)
    - [Улучшение нашего I/O проекта](ch13-03-improving-our-io-project.md)
    - [Производительность](ch13-04-performance.md)

- [Больше о Cargo и Crates.io](ch14-00-more-about-cargo.md)
    - [Выборочная сборка с профилями выпуска](ch14-01-release-profiles.md)
    - [Публикация библиотеки в Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Рабочие области Cargo](ch14-03-cargo-workspaces.md)
    - [Установка бинарных файлов из Crates.io с помощью `cargo install`](ch14-04-installing-binaries.md)
    - [Расширение Cargo пользовательскими коммандами](ch14-05-extending-cargo.md)

- [Умные указатели](ch15-00-smart-pointers.md)
    - [Тип данных `Box<T>` ссылается на данные в куче и имеет известный размер](ch15-01-box.md)
    - [Типаж `Deref` позволяет получить доступ к данным по ссылке](ch15-02-deref.md)
    - [Типаж `Drop` выполняется когда значение становится недействительным](ch15-03-drop.md)
    - [Счётчик указателей `Rc<T>`](ch15-04-rc.md)
    - [`RefCell<T>` и шабон внутренней изменяемости](ch15-05-interior-mutability.md)
    - [Защита и создание ссылочного зацикливания и утечки памяти](ch15-06-reference-cycles.md)

- [Многопоточность без страха](ch16-00-concurrency.md)
    - [Потоки](ch16-01-threads.md)
    - [Отправление сообщений потокам](ch16-02-message-passing.md)
    - [Общее состояние](ch16-03-shared-state.md)
    - [Расширение возможностей многопоточных программ с помощью типажей `Sync` и `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Является ли Rust ООП языком программирования?](ch17-00-oop.md)
    - [Что означает обьетно-ориентированный?](ch17-01-what-is-oo.md)
    - [Обьекты-типажи для использования значений разных типов](ch17-02-trait-objects.md)
    - [Реализация ООП шаблона проектирования](ch17-03-oo-design-patterns.md)

## Расширеные темы

- [Шаблоны соответствуют структуре значений](ch18-00-patterns.md)
    - [Места использования шаблонов](ch18-01-all-the-places-for-patterns.md)
    - [Когда шаблон может не сработать](ch18-02-refutability.md)
    - [Синтаксис шаблонов](ch18-03-pattern-syntax.md)

- [Расширенные возможности](ch19-00-advanced-features.md)
    - [Небезопасный Rust](ch19-01-unsafe-rust.md)
    - [Расширенные модификаторы времени жизни](ch19-02-advanced-lifetimes.md)
    - [Расширенные опции типажей](ch19-03-advanced-traits.md)
    - [Расширенные типы](ch19-04-advanced-types.md)
    - [Дополнительные сведения о функциях и замыканиях](ch19-05-advanced-functions-and-closures.md)

- [Заключительный проект: реализация многопоточного веб-сервера](ch20-00-final-project-a-web-server.md)
    - [Однопоточный веб-сервер](ch20-01-single-threaded.md)
    - [Как медленные запросы влияют на пропускную способность](ch20-02-slow-requests.md)
    - [Проектирование интерфейса пула потоков](ch20-03-designing-the-interface.md)
    - [Создание пула потоков и сохранение в него потоков](ch20-04-storing-threads.md)
    - [Отправка запросов потокам с помощью каналов](ch20-05-sending-requests-via-channels.md)
    - [Изящное завершение и освобождение ресурсов](ch20-06-graceful-shutdown-and-cleanup.md)

- [Дополнительная информация](appendix-00.md)
    - [Дополнение A: Ключевые слова](appendix-01-keywords.md)
    - [Дополнение B: Операторы](appendix-02-operators.md)
<!--
    - [C - Derivable Traits]()
    - [D - Nightly Rust]()
    - [E - Macros]()
    - [F - Translations]()
-->
    - [Дополнение G: Новые возможности](appendix-07-newest-features.md)
