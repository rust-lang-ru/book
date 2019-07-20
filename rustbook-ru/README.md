# Язык программирования Rust

Данный репозиторий содержит перевод второго издания “Язык программирования Rust”. 

Второе издание - это переработанная книга "The Rust Programming Language", которая будет напечатана издательством "No Starch Press" ориентировочно в мае 2018 года. Последнюю информацию о дате выхода книги и о способе ее заказа вы можете узнать на официальном сайте издательства [No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

Книгу можно [читать онлайн](https://rustycrate.ru/book).

Оригинал книги вы можете прочесть [онлайн][html]; несколько последних глав еще не закончены, но готовая часть книги заметно улучшена по сравнению с первым изданием. Авторы оригинальной книги рекомендуют начать чтение со второго издания.

[html]: http://rust-lang.github.io/book/

## Требования

Для сборки книги требуется [mdBook] >= v0.0.13. Для установки выполните команду:

[mdBook]: https://github.com/rust-lang-nursery/mdBook/

```bash
$ cargo install mdbook
```

## Сборка

Для того, чтобы скомпилировать книгу, перейдите в нужный каталог с помощью команды cd - first-edition для первого, либо second-edition для второго издания.
Далее введите следующую команду:

```bash
$ mdbook build
```

Результаты выполнения команды появятся в подкаталоге `book`. Для проверки откройте книгу в браузере.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Для запуска тестов:

```bash
$ mdbook test
```


# Полезные ссылки

Чаты                                   | Ссылки
---------------------------------------|--------
для обсуждения языка, получения помощи | [![Join the chat at https://gitter.im/ruRust/general](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/ruRust/general?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
для обсуждения самой книги и вопросов перевода | [![Join the chat at https://gitter.im/ruRust/rust_book_ru](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/ruRust/rust_book_ru?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![ruRust/rust_book_ru](http://issuestats.com/github/ruRust/rust_book_ru/badge/pr?style=flat)](http://issuestats.com/github/ruRust/rust_book_ru)
[![ruRust/rust_book_ru](http://issuestats.com/github/ruRust/rust_book_ru/badge/issue?style=flat)](http://issuestats.com/github/ruRust/rust_book_ru)


# Соавторам

## С чего начать

При желании помочь с переводом пишите в группу, указанную выше. Вам ответят на любые вопросы по теме.
Нам особенно интересна помощь именно со вторым изданием! 
Не бойтесь code review, у нас не принято наезжать на новичков. :smile:

## Для опытных

[Правила перевода](https://github.com/ruRust/rust_book_ru/wiki/Правила).

## Ресурсы
* первое издание rustbook расположено [здесь][original]
* перевод первого издания расположен [здесь][rustbook]

[rustbook]: http://ruRust.github.io/rust_book_ru
[original]: https://doc.rust-lang.org/book/first-edition/
