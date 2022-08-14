# Финальный проект: создание многопоточного веб-сервера

Это был долгий путь, но мы дошли до финала книги. В этой главе мы создадим ещё один проект для демонстрации некоторых концепций, которые мы рассмотрели в последних главах, а также резюмировать некоторые предыдущие уроки.

Для нашего финального проекта мы создадим веб-сервер, который говорит “hello” и выглядит как рисунке 20-1 в веб-браузере.

![hello from rust](https://github.com/ruRust/book/blob/master/rustbook-en/src/img/trpl20-01.png?raw=true)

<span class="caption">Рисунок 20-1: Наш последний совместный проект</span>

Вот наш план для создания веб-сервера:

1. Узнать немного о протоколах TCP и HTTP.
2. Прослушивать TCP соединения у сокета.
3. Разобрать небольшое количество HTTP-запросов.
4. Создать правильный HTTP ответ.
5. Улучшите пропускную способность нашего сервера с помощью пула потоков.

Before we get started, we should mention one detail: the method we’ll use won’t be the best way to build a web server with Rust. Community members have published a number of production-ready crates available on [crates.io](https://crates.io/) that provide more complete web server and thread pool implementations than we’ll build. However, our intention in this chapter is to help you learn, not to take the easy route. Because Rust is a systems programming language, we can choose the level of abstraction we want to work with and can go to a lower level than is possible or practical in other languages. We’ll therefore write the basic HTTP server and thread pool manually so you can learn the general ideas and techniques behind the crates you might use in the future.
