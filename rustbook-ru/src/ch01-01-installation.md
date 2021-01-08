## Установка

Первым шагом является установка Rust. Мы загрузим Rust, используя инструмент командной строки `rustup`, предназначенный для управлениями версиями Rust и другими связанными с ним инструментами. Вам понадобится интернет соединение для его загрузки.

> Замечание: Если вы предпочтёте не использовать `rustup` по какой-то причине, пожалуйста, ознакомьтесь с другими вариантами на с странице [the Rust installation page](https://www.rust-lang.org/tools/install).

Следующие шаги устанавливают последнюю стабильную версию компилятора Rust. Стабильность Rust гарантирует, что все примеры в книге, которые компилируются, будут продолжать компилироваться с более новыми версиями Rust. Вывод может немного отличаться между версиями, потому что Rust часто улучшает сообщения об ошибках и предупреждения. Другими словами, любая более новая стабильная версия Rust, которую вы устанавливаете с помощью этих шагов, должна работать должным образом с содержанием этой книги.

> ### Нотация командной строки
> In this chapter and throughout the book, we’ll show some commands used in the terminal. Lines that you should enter in a terminal all start with `$`. You don’t need to type in the `$` character; it indicates the start of each command. Lines that don’t start with `$` typically show the output of the previous command. Additionally, PowerShell-specific examples will use `>` rather than `$`.

### Установка `rustup` на Linux или macOS

Если вы используете Linux или macOS, пожалуйста, выполните следующую команду:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup` tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```text
Rust is installed now. Great!
```

Additionally, you’ll need a linker of some kind. It’s likely one is already installed, but when you try to compile a Rust program and get errors indicating that a linker could not execute, that means a linker isn’t installed on your system and you’ll need to install one manually. C compilers usually come with the correct linker. Check your platform’s documentation for how to install a C compiler. Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one now.

### Установка `rustup` на Windows

On Windows, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire the build tools is to install [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/visual-cpp-build-tools/). When asked which workloads to install make sure "C++ build tools" is selected and that the Windows 10 SDK and the English language pack components are included.

The rest of this book uses commands that work in both *cmd.exe* and PowerShell. If there are specific differences, we’ll explain which to use.

### Обновление и удаление

После установки Rust с помощью `rustup`, обновление на последние версии выполняется с помощью следующего простого скрипта командой:

```console
$ rustup update
```

Чтобы удалить Rust и `rustup`, выполните<br>следующую команду:

```console
$ rustup self uninstall
```

### Устранение возможных ошибок

Чтобы проверить, правильно ли у вас установлен Rust, откройте оболочку и введите эту строку:

```console
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest stable version that has been released in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t see this information and you’re on Windows, check that Rust is in your `%PATH%` system variable. If that’s all correct and Rust still isn’t working, there are a number of places you can get help. The easiest is the #beginners channel on [the official Rust Discord](https://discord.gg/rust-lang). There, you can chat with other Rustaceans (a silly nickname we call ourselves) who can help you out. Other great resources include [the Users forum](https://users.rust-lang.org/) and [Stack Overflow](https://stackoverflow.com/questions/tagged/rust).

### Локальная документация

Установка Rust также включает локальную копию документации, поэтому вы можете читать её в автономном режиме. Запустите `rustup doc`, чтобы открыть локальную документацию в вашем браузере.

Any time a type or function is provided by the standard library and you’re not sure what it does or how to use it, use the application programming interface (API) documentation to find out!
