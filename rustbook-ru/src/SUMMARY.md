# The Ржавчина Programming Language

[Язык программирования Ржавчина](title-page.md) [Предисловие](foreword.md) [Введение](ch00-00-introduction.md)

## С чего начать

- [С чего начать](ch01-00-getting-started.md)

    - [Установка](ch01-01-installation.md)
    - [Привет, Мир!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Программирование игры в загадки](ch02-00-guessing-game-tutorial.md)

- [Общие подходы программирования](ch03-00-common-programming-concepts.md)

    - [Переменные и изменяемость](ch03-01-variables-and-mutability.md)
    - [Виды Данных](ch03-02-data-types.md)
    - [Функции](ch03-03-how-functions-work.md)
    - [Примечания](ch03-04-comments.md)
    - [Круговороты](ch03-05-control-flow.md)

- [Понимание владения](ch04-00-understanding-ownership.md)

    - [Что такое "владение"?](ch04-01-what-is-ownership.md)
    - [Ссылки и заимствование](ch04-02-references-and-borrowing.md)
    - [Вид среза](ch04-03-slices.md)

- [Использование стопок для объединения связанных данных](ch05-00-structs.md)

    - [Определение и создание образцов стопок](ch05-01-defining-structs.md)
    - [Пример программы, использующей устройства](ch05-02-example-structs.md)
    - [Правила написания способов](ch05-03-method-syntax.md)

- [Перечисления и сопоставление с образцом](ch06-00-enums.md)

    - [Определение Enum](ch06-01-defining-an-enum.md)
    - [Устройство потока управления `match`](ch06-02-match.md)
    - [Краткий поток управления с `if let`](ch06-03-if-let.md)

## Основы Ржавчины

- [Управление растущими делами с помощью дополнений, ящиков и разделов](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)

    - [Дополнения и ящики](ch07-01-packages-and-crates.md)
    - [Определение разделов для управления областью действия и тайностью](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Пути для ссылки на переменную в дереве разделов](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Введение путей в область видимости с помощью ключевого слова `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Разделение разделов на разные файлы](ch07-05-separating-modules-into-different-files.md)

- [Общее: Собрания](ch08-00-common-collections.md)

    - [Хранение списков значений с векторами](ch08-01-vectors.md)
    - [Хранение закодированного писания UTF-8 со строками](ch08-02-strings.md)
    - [Хранение ключей со связанными значениями в HashMap](ch08-03-hash-maps.md)

- [Обработка ошибок](ch09-00-error-handling.md)

    - [Неустранимые ошибки с `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Устранимые ошибки с `Result`](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` или Не `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Общие виды данных, сущности (свойства) и время жизни](ch10-00-generics.md)

    - [Обобщённые виды данных](ch10-01-syntax.md)
    - [Сущности (свойства): определение разделяемого поведения](ch10-02-traits.md)
    - [Проверка действительности ссылок посредством сроков жизни](ch10-03-lifetime-syntax.md)

- [Написание самостоятельно х проверок](ch11-00-testing.md)

    - [Как писать проверки](ch11-01-writing-tests.md)
    - [Управление выполнением проверок](ch11-02-running-tests.md)
    - [Создание проверок](ch11-03-test-organization.md)

- [Дело с вводом-выводом: создание программы приказной строки](ch12-00-an-io-project.md)

    - [Получение переменных приказной строки](ch12-01-accepting-command-line-arguments.md)
    - [Чтение файла](ch12-02-reading-a-file.md)
    - [Переработка рукописи для обеспечения выделения на разделы и улучшения обработки ошибок](ch12-03-improving-error-handling-and-modularity.md)
    - [Разработка возможности библиотеки с помощью разработки через проверку](ch12-04-testing-the-librarys-functionality.md)
    - [Работа с переменными среды](ch12-05-working-with-environment-variables.md)
    - [Запись сообщений об ошибках в stderr вместо stdout](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Думать на Ржавчины

- [Полезные  возможности языка: повторители и замыкания](ch13-00-functional-features.md)

    - [Замыкания: анонимные функции, которые захватывают своё окружение](ch13-01-closures.md)
    - [Обработка последовательности переменных с помощью повторителей](ch13-02-iterators.md)
    - [Улучшение нашего дела с вводом/выводом](ch13-03-improving-our-io-project.md)
    - [Сравнение производительности: круговороты и повторители](ch13-04-performance.md)

- [Подробнее о Cargo и Crates.io](ch14-00-more-about-cargo.md)

    - [Настройка сборок с помощью профилей выпуска](ch14-01-release-profiles.md)
    - [Обнародование ящика на Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Рабочие области Cargo](ch14-03-cargo-workspaces.md)
    - [Установка двоичных файлов с Crates.io с помощью `cargo install`](ch14-04-installing-binaries.md)
    - [Расширение возможностей Cargo путём добавления пользовательских приказов](ch14-05-extending-cargo.md)

- [Умные указатели](ch15-00-smart-pointers.md)

    - [Использование `Box<T>` для указания на данные в куче](ch15-01-box.md)
    - [Работа с умными указателями как с обычными ссылками с помощью сущности `Deref`](ch15-02-deref.md)
    - [Выполнение рукописи при очистке с помощью сущности `Drop`](ch15-03-drop.md)
    - [`Rc<T>`, умный указатель с подсчётом ссылок](ch15-04-rc.md)
    - [`RefCell<T>` и внутренняя изменяемость](ch15-05-interior-mutability.md)
    - [Ссылочные круговороты могут привести к утечке памяти](ch15-06-reference-cycles.md)

- [Безбоязненный одновременность](ch16-00-concurrency.md)

    - [Использование потоков для одновременного выполнения рукописи](ch16-01-threads.md)
    - [Пересылка сообщений для передачи данных между потоками](ch16-02-message-passing.md)
    - [Одновременность с общим состоянием](ch16-03-shared-state.md)
    - [Расширяемый одновременность с помощью сущностей `Sync` и `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Возможности предметно-направленного программирования Ржавчина](ch17-00-oop.md)

    - [Свойства предметно-направленных языков](ch17-01-what-is-oo.md)
    - [Использование сущность-предметов, допускающих значения разных видов данных](ch17-02-trait-objects.md)
    - [Выполнение образца данных предметно-направленной разработки](ch17-03-oo-design-patterns.md)

## Продвинутые сути

- [Образцы и сопоставление](ch18-00-patterns.md)

    - [Все места, где могут использоваться образцы](ch18-01-all-the-places-for-patterns.md)
    - [Опровержимость: может ли образец данных не соответствовать](ch18-02-refutability.md)
    - [Правила написания образца](ch18-03-pattern-syntax.md)

- [Расширенные возможности](ch19-00-advanced-features.md)

    - [Небезопасная рукопись в Ржавчине](ch19-01-unsafe-rust.md)
    - [Продвинутые сущности](ch19-03-advanced-traits.md)
    - [Продвинутые виды данных](ch19-04-advanced-types.md)
    - [Продвинутые функции и замыкания](ch19-05-advanced-functions-and-closures.md)
    - [Макросы](ch19-06-macros.md)

- [Конечный дело: создание многопоточного сетевого-отдельного вычислителя](ch20-00-final-project-a-web-server.md)

    - [Создание однопоточного сетевого-отдельного вычислителя](ch20-01-single-threaded.md)
    - [Превращение нашего однопоточного отдельного вычислителя в многопоточный отдельный вычислитель](ch20-02-multithreaded.md)
    - [	Мягкое завершение работы и очистка](ch20-03-graceful-shutdown-and-cleanup.md)

- [Приложения](appendix-00.md)

    - [А — Ключевые слова](appendix-01-keywords.md)
    - [B — Приказчики и знаки](appendix-02-operators.md)
    - [C — Выводимые сущности](appendix-03-derivable-traits.md)
    - [D — Полезные средства разработки](appendix-04-useful-development-tools.md)
    - [E — Издания](appendix-05-editions.md)
    - [F — Переводы книги](appendix-06-translation.md)
    - [G — Как создаётся Ржавчина и «Ночное издание Ржавчины»](appendix-07-nightly-rust.md)
