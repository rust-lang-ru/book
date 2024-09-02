# Язык программирования Ржавчина

<<<<<<< HEAD
Данный хранилище содержит перевод второго издания “Язык программирования Ржавчина”. 
=======
Данное хранилище содержит перевод второго издания “Язык программирования Ржавчина”. 
>>>>>>> 8026f7cc37a623b7a0968ae296cbb5f4118ea196

Второе издание - это переработанная книга "The Ржавчина Programming Language", которая будет напечатана издательством "No Starch Press" примерно в мае 2018 года. Последнюю сведения о дате выхода книги и о способе ее заказа вы можете узнать на сайте самого издательства [No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

Книгу можно [читать в сети](https://rustycrate.ru/book).

Подлинник книги вы можете прочесть [в сети][html]; несколько последних глав еще не закончены, но готовая часть книги заметно улучшена по сравнению с первым изданием. Составители изначальной книги советуют начать чтение со второго издания.

[html]: http://rust-lang.github.io/book/

## Требования

Для сборки книги требуется [mdBook] >= v0.0.13. Для установки выполните приказ:

[mdBook]: https://github.com/rust-lang-nursery/mdBook/

```bash
$ cargo install mdbook
```

## Сборка

<<<<<<< HEAD
Для того, чтобы собрать книгу, перейдите в нужный папка с помощью приказы cd - first-edition для первого, либо second-edition для второго издания.
=======
Для того, чтобы собрать книгу, перейдите в нужную папку с помощью приказов cd - first-edition для первого, либо second-edition для второго издания.
>>>>>>> 8026f7cc37a623b7a0968ae296cbb5f4118ea196
Далее введите следующий приказ:

```bash
$ mdbook build
```

<<<<<<< HEAD
Итоги выполнения приказы появятся в подпапке `book`. Для проверки откройте книгу в обозревателе.
=======
Итогом сборки книги будет являться созданная подпапка `book`и все ее содержимое. Для проверки откройте книгу в любом обозревателе.
>>>>>>> 8026f7cc37a623b7a0968ae296cbb5f4118ea196

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

Для запуска проверок:

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


# Сосоставителям

## С чего начать

При желании помочь с переводом - для этого пишите в объединение, указанное выше. Вам ответят на любые вопросы по теме.
Нам особенно важна помощь именно со вторым изданием! 
Не бойтесь code review, у нас не принято наезжать на новичков. :smile:

## Для опытных

[Правила перевода](https://github.com/ruRust/rust_book_ru/wiki/Правила).

## Источники
<<<<<<< HEAD
* первое издание rustbook расположено [здесь][original]
=======
* первое издание пособия по Ржавчине расположено [здесь][original]
>>>>>>> 8026f7cc37a623b7a0968ae296cbb5f4118ea196
* перевод первого издания расположен [здесь][rustbook]

[rustbook]: http://ruRust.github.io/rust_book_ru
[original]: https://doc.rust-lang.org/book/first-edition/
