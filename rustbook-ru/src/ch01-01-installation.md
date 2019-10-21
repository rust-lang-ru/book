## Установка

The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.

> Замечание: Если по каким-то причинам вы предпочтете не использовать `rustup`, то загляните на страницу [the Rust installation](https://www.rust-lang.org/tools/install) для других вариантов.

The following steps install the latest stable version of the Rust compiler.
Rust’s stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions, because Rust often improves error messages
and warnings. In other words, any newer, stable version of Rust you install
using these steps should work as expected with the content of this book.

> ### Нотация командной строки
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type in the `$` character; it indicates the start of each
> command. Lines that don’t start with `$` typically show the output of the
> previous command. Additionally, PowerShell-specific examples will use `>`
> rather than `$`.

### Установка на Linux или Mac

Если вы используете Linux или Mac, пожалуйста, выполните следующую команду:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

Команда скачивает скрипт, который начинает установку первого  инструмента `rustup`, предназначенного для дальнейшей установки последней стабильной версии Rust. Вас могут запросить ввести локальный пароль. При успешной установке вы увидите следующий вывод:

```text
Rust is installed now. Great!
```

Если хотите, можете скачать данный скрипт и изучить его перед запуском.

Инсталлятор автоматически добавит папку с компонентами Rust в системный путь PATH
и их использование будет доступно из командной строки после следующего входа в
систему. Если необходимо сразу начать работу, пожалуйста, выполните следующую команду:

```text
$ source $HOME/.cargo/env
```

Или добавьте следующую строку в файл `~/.bash_profile`:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Additionally, you’ll need a linker of some kind. It’s likely one is already
installed, but when you try to compile a Rust program and get errors indicating
that a linker could not execute, that means a linker isn’t installed on your
system and you’ll need to install one manually. C compilers usually come with
the correct linker. Check your platform’s documentation for how to install a C
compiler. Also, some common Rust packages depend on C code and will need a C
compiler. Therefore, it might be worth installing one now.

### Установка на Windows

On Windows, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow
the instructions for installing Rust. At some point in the installation, you’ll
receive a message explaining that you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2019](https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2019). The tools are in
the Other Tools and Frameworks section.

Остальные часть книги использует команды работающие как в *cmd.exe* так и в PowerShell. Есть некоторые отличия, которые мы объясним.

### Обновление

После установки Rust с помощью `rustup`, обновление на последние версии выполняется с помощью следующего простого скрипта командой:

```text
$ rustup update
```

Удалить Rust также просто, как его установить. Для этого необходим выполнить
следующую команду:

```text
$ rustup self uninstall
```

### Устранение возможных ошибок

Проверка работы Rust-компилятора:

```text
$ rustc --version
```

Вы должны увидеть версию компилятора, хэш-строку, дату последней стабильной версии обновления в формате:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Если проблема всё ещё не может быть устранена, вот сайты, на страницах которых Вы
сможете найти ответ или задать вопрос [the #rust IRC channel on irc.mozilla.org](irc://irc.mozilla.org/#rust).<comment>,
Доступ к данному ресурсу осуществляется с помощь <a href="http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust" data-md-type="link">Mibbit</a>. Перейдите по
ссылке и Вы получите возможность задать Ваш вопрос в чате. Также обратить на форум
<a href="https://users.rust-lang.org/" data-md-type="link">the Users forum</a> или на <a href="http://stackoverflow.com/questions/tagged/rust" data-md-type="link">Stack Overflow</a>.</comment>

### Локальная документация

The installation of Rust also includes a copy of the documentation locally, so
you can read it offline. Run `rustup doc` to open the local documentation in
your browser.

Если Вам необходимо найти описание типа или функции в стандартной библиотеке или вы не знаете, что она делает или как его использовать, то документация станет вашим верным помощником!
