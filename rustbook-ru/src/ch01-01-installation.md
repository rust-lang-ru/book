## Установка

Первым шагом является установка Rust. Мы загрузим Rust, используя инструмент командной строки `rustup`, предназначенный для управлениями версиями Rust и другими связанными с ним инструментами. Вам понадобится интернет соединение для его загрузки.

> Note: If you prefer not to use `rustup` for some reason, please see [the Rust installation page](https://www.rust-lang.org/tools/install) for other options.

The following steps install the latest stable version of the Rust compiler. Rust’s stability guarantees ensure that all the examples in the book that compile will continue to compile with newer Rust versions. The output might differ slightly between versions, because Rust often improves error messages and warnings. In other words, any newer, stable version of Rust you install using these steps should work as expected with the content of this book.

> ### Command Line Notation
> В данной главе и потом во всей книге, мы покажем некоторые команды в терминале командной строки. Строки, которые нужно ввести в терминале начинаются с `$`. Но вам не нужно вводить сам символ  `$`; он только отображает, что это начало каждой команды. Строки, которые НЕ начинаются с `$`, обычно показывают вывод предыдущей команды. В дополнение, примеры специфичные для PowerShell используют символ `>` вместо символа `$`.

### Installing `rustup` on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Команда скачивает скрипт, который начинает установку первого  инструмента `rustup`, предназначенного для дальнейшей установки последней стабильной версии Rust. Вас могут запросить ввести локальный пароль. при успешной установке вы увидите следующий вывод:

```text
Rust is installed now. Great!
```

Additionally, you’ll need a linker of some kind. It’s likely one is already installed, but when you try to compile a Rust program and get errors indicating that a linker could not execute, that means a linker isn’t installed on your system and you’ll need to install one manually. C compilers usually come with the correct linker. Check your platform’s documentation for how to install a C compiler. Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one now.

### Installing `rustup` on Windows

On Windows, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire the build tools is to install [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/visual-cpp-build-tools/). When asked which workloads to install make sure "C++ build tools" is selected and that the Windows 10 SDK and the English language pack components are included.

Остальные часть книги использует команды работающие как в *cmd.exe* так и в PowerShell. Есть некоторые отличия, которые мы объясним.

### Updating and Uninstalling

После установки Rust с помощью `rustup`, обновление на последние версии выполняется с помощью следующего простого скрипта командой:

```console
$ rustup update
```

Чтобы удалить Rust и `rustup`, выполните<br>следующую команду:

```console
$ rustup self uninstall
```

### Troubleshooting

Чтобы проверить, правильно ли у вас установлен Rust, откройте оболочку и введите эту строку:

```console
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest stable version that has been released in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t see this information and you’re on Windows, check that Rust is in your `%PATH%` system variable. If that’s all correct and Rust still isn’t working, there are a number of places you can get help. The easiest is the #beginners channel on [the official Rust Discord](https://discord.gg/rust-lang). There, you can chat with other Rustaceans (a silly nickname we call ourselves) who can help you out. Other great resources include [the Users forum](https://users.rust-lang.org/) and [Stack Overflow](https://stackoverflow.com/questions/tagged/rust).

### Local Documentation

The installation of Rust also includes a copy of the documentation locally, so you can read it offline. Run `rustup doc` to open the local documentation in your browser.

Каждый раз, когда тип или функция предоставляется из стандартной библиотеки и вы не знаете, что он делает или как его использовать, используйте документацию по интерфейсу прикладного программирования (API), чтобы узнать!
