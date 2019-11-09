## Hello, World!

Итак, когда Rust уже установлен можно приступать к написанию вашей первой программы.
[По традиции](https://ru.wikipedia.org/wiki/Hello,_world!), (а точнее с 1978 года, когда вышло в свет первое издание [книги о Си](https://ru.wikipedia.org/wiki/%D0%AF%D0%B7%D1%8B%D0%BA_%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D0%BC%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F_%D0%A1%D0%B8_(%D0%BA%D0%BD%D0%B8%D0%B3%D0%B0)))
напишем небольшую программу, которая напечатает "Привет, Мир!" в строке вывода.

> Обратите внимание, книга подразумевает, что читатели должны быть знакомы с командной строкой. Язык Rust не требует каких-то специальных редакторов исходного кода или инструментальных средств. Если вы предпочитаете использовать IDE (integrated development environment), вместо утилит командной строки, то можете пробовать свой любимый IDE. Многие IDE имеют разный уровень поддержки Rust, читайте детали в документации про IDE. Не так давно команда Rust фокусировалась на большей поддержке IDE и сделала определенный прогресс в данном направлении!

### Создание папки проекта

Вы начнете с создания папки для хранения исходного кода Rust. Для Rust не важно где он находится, но для упражнений и проектов из книги, мы советуем создать директорию *projects* в домашнем каталоге и хранить их там.

Откройте терминал и введите следующие команды для создания каталога *projects* и подкаталога для проекта “Hello, world!” внутри *projects*.

For Linux, macOS, and PowerShell on Windows, enter this:

```text
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

For Windows CMD, enter this:

```cmd
> mkdir %USERPROFILE%\projects
> cd %USERPROFILE%\projects
> mkdir hello_world
> cd hello_world
```

### Написание и запуск первой программы

Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, use *hello_world.rs* rather than
*helloworld.rs*.

Теперь откроем файл *main.rs* для редактирования и введём следующие строки кода:

<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("Hello, Rust world! Привет, Мир!");
}
```

<span class="caption">Listing 1-1: A program that prints <code>Hello, world!</code></span>

Save the file and go back to your terminal window. On Linux or macOS, enter
the following commands to compile and run the file:

```text
$ rustc main.rs # нажмите клавишу Enter
$ ./main # нажмите клавишу Enter
Hello, Rust world! Привет, Мир!
```

On Windows, enter the command `.\main.exe` instead of `./main`:

```powershell
> rustc main.rs # нажмите клавишу Enter
> main
Hello, Rust world! Привет, Мир!
```

Regardless of your operating system, the string `Hello, world!` should print to
the terminal. If you don’t see this output, refer back to the
[“Troubleshooting”](ch01-01-installation.html#troubleshooting)<comment> part of the Installation
section for ways to get help.</comment>

If `Hello, world!` did print, congratulations! You’ve officially written a Rust
program. That makes you a Rust programmer—welcome!

### Anatomy of a Rust Program

Let’s review in detail what just happened in your “Hello, world!” program.
Here’s the first piece of the puzzle:

```rust
fn main() {

}
```

These lines define a function in Rust. The `main` function is special: it is
always the first code that runs in every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses, `()`.

Также заметим, что тело функции обернуто в фигурные скобки (curly brackets) `{}`. В Rust они требуются вокруг всех тел функций. Хорошим стилем является размещение открывающей скобки в строке объявления функции, оставляя пробел между ними.

At the time of this writing, an automatic formatter tool called `rustfmt` is
under development. If you want to stick to a standard style across Rust
projects, `rustfmt` will format your code in a particular style. The Rust team
plans to eventually include this tool with the standard Rust distribution, like
`rustc`. So depending on when you read this book, it might already be installed
on your computer! Check the online documentation for more details.

Содержимое функции `main`:

```rust
    println!("Hello, Rust world! Привет, Мир!");
```

This line does all the work in this little program: it prints text to the
screen. There are four important details to notice here. First, Rust style is
to indent with four spaces, not a tab.

Второе, вызов `println!` является макросом. Если бы вызывалась функция, то она была бы введена как `println` (без восклицательного знака `!`). Более детальное обсуждение макросов Rust будет в главе 19. Сейчас вам достаточно знать, что использование символа `!` означает вызов макроса, а не обычной функции.

Third, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.

Fourth, we end the line with a semicolon (`;`), which indicates that this
expression is over and the next one is ready to begin. Most lines of Rust code
end with a semicolon.

### Compiling and Running Are Separate Steps

You’ve just run a newly created program, so let’s examine each step in the
process.

Before running a Rust program, you must compile it using the Rust compiler by
entering the `rustc` command and passing it the name of your source file, like
this:

```text
$ rustc main.rs
```

If you have a C or C++ background, you’ll notice that this is similar to `gcc`
or `clang`. After compiling successfully, Rust outputs a binary executable.

On Linux, macOS, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell. On Linux and macOS, you’ll see two
files. With PowerShell on Windows, you’ll see the same three files that you
would see using CMD.

```text
$ ls
main  main.rs
```

With CMD on Windows, you would enter the following:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

This shows the source code file with the *.rs* extension, the executable file
(*main.exe* on Windows, but *main* on all other platforms), and, when using
Windows, a file containing debugging information with the *.pdb* extension.
From here, you run the *main* or *main.exe* file, like this:

```text
$ ./main # or .\main.exe on Windows
```

If *main.rs* was your “Hello, world!” program, this line would print `Hello, world!` to your terminal.

Если вы знакомы с динамическими языками вроде Ruby, Python или
JavaScript, то возможно вы не использовали компиляцию и запуск программы отдельными шагами. Rust является "заранее скомпилированным" ( *ahead-of-time compiled* ) языком. Это означает, что можно скомпилировать программу и передать исполняемый файл еще кому то, а они могут могут его запустить, не имея локально установленного Rust. А если вы отдаете файл *.rb*, *.py* или *.js*, то им необходимо иметь установленную реализацию для Ruby, Python или JavaScript соответственно. В этих языках нужна только одна команда для компиляции и запуска программы. Все является компромиссом в дизайне языков.

Только компиляция с помощью `rustc` подходит только для простой программы. По мере того как проект растет, вам захотите управлять всеми опциями и упростить обмен своим кодом с другими. Далее, мы представим инструмент Cargo, который поможет в написании настоящих программ на Rust.
