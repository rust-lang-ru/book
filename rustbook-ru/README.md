# Язык программирования Ржавчина

Данное хранилище содержит перевод второго издания “Язык программирования Ржавчина”. 

Второе издание - это переработанная книга "The Ржавчина язык программирования", которая будет напечатана издательством "No Starch Press" примерно в мае 2018 года. Последнюю сведения о дате выхода книги и о способе ее заказа вы можете узнать на сайте самого издательства [No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

Книгу можно [Прочесть в сети](https://rustycrate.ru/book).

Подлинник книги вы можете прочесть [в сети][html]; несколько последних глав еще не закончены, но готовая часть книги заметно улучшена по сравнению с первым изданием. Составители изначальной книги советуют начать чтение со второго издания.

[html]: http://rust-lang.github.io/book/

## Требования

Сборка книги требует библиотеку [mdBook], лучше всего - когда это будет то же исполнение, которое
rust-lang/rust использует [этом файле][rust-mdbook]. Чтобы получить её:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

Книга также использует два расширения mdbook, которые являются частью этого репозитория. Если вы их не установите, вы увидите предупреждения при сборке, и вывод будет выглядеть неправильно и искаженно, но вы *все равно* сможете собрать книгу. Чтобы использовать расширения,вы должны запустить:

```bash
$ cargo install --locked --path packages/mdbook-trpl-listing
$ cargo install --locked --path packages/mdbook-trpl-note
```

## Сборка

Для того, чтобы собрать книгу, перейдите в нужную папку с помощью приказов cd - first-edition для первого, либо second-edition для второго издания.
Далее введите следующий приказ:

```bash
$ mdbook build
```

Итогом сборки книги будет являться созданная подпапка `book`и все ее содержимое. Для проверки откройте книгу в любом обозревателе.

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
* первое издание пособия по Ржавчине расположено [здесь][original]
* перевод первого издания расположен [здесь][rustbook]

[rustbook]: http://ruRust.github.io/rust_book_ru
[original]: https://doc.rust-lang.org/book/first-edition/
