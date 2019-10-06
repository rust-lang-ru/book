# Введение

> Примечание: это издание книги так же, как и [The Rust Programming Language](https://nostarch.com/rust) доступно в печатном и электронном виде от [No Starch Press](https://nostarch.com/) .

Welcome to *The Rust Programming Language*, an introductory book about Rust.
The Rust programming language helps you write faster, more reliable software.
High-level ergonomics and low-level control are often at odds in programming
language design; Rust challenges that conflict. Through balancing powerful
technical capacity and a great developer experience, Rust gives you the option
to control low-level details (such as memory usage) without all the hassle
traditionally associated with such control.

## Для кого Rust

Rust подходит для многих людей по разным причинам. Приведем несколько самых важных групп.

### Команды разработчиков

Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Low-level code
is prone to a variety of subtle bugs, which in most other languages can be
caught only through extensive testing and careful code review by experienced
developers. In Rust, the compiler plays a gatekeeper role by refusing to
compile code with these elusive bugs, including concurrency bugs. By working
alongside the compiler, the team can spend their time focusing on the program’s
logic rather than chasing down bugs.

Rust также предлагает современные инструменты для разработчиков в мире системного программирования:

- Cargo, встроенный менеджер зависимостей и инструмент сборки,  добавляет, компилирует и управляет зависимостями безболезненно и согласованно, используя экосистему Rust.
- Rustfmt обеспечивает согласованный стиль кодирования для всех разработчиков.
- Rust Language Server поддерживает интегрированную среду разработки (IDE) с автодополнением кода и встроенными сообщениями об ошибках.

Эти и другие инструменты экосистемы Rust, обеспечивают  разработчикам продуктивность при написании кода системного уровня.

### Студенты

Rust is for students and those who are interested in learning about systems
concepts. Using Rust, many people have learned about topics like operating
systems development. The community is very welcoming and happy to answer
student questions. Through efforts such as this book, the Rust teams want to
make systems concepts more accessible to more people, especially those new to
programming.

### Компании

Hundreds of companies, large and small, use Rust in production for a variety of
tasks. Those tasks include command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, Internet of Things applications, machine
learning, and even major parts of the Firefox web browser.

### Разработчики Open Source

Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We’d love to have you contribute to the Rust
language.

### Люди, которые ценят скорость и стабильность

Rust is for people who crave speed and stability in a language. By speed, we
mean the speed of the programs that you can create with Rust and the speed at
which Rust lets you write them. The Rust compiler’s checks ensure stability
through feature additions and refactoring. This is in contrast to the brittle
legacy code in languages without these checks, which developers are often
afraid to modify. By striving for zero-cost abstractions, higher-level features
that compile to lower-level code as fast as code written manually, Rust
endeavors to make safe code be fast code as well.

The Rust language hopes to support many other users as well; those mentioned
here are merely some of the biggest stakeholders. Overall, Rust’s greatest
ambition is to eliminate the trade-offs that programmers have accepted for
decades by providing safety *and* productivity, speed *and* ergonomics. Give
Rust a try and see if its choices work for you.

## Для кого эта книга

This book assumes that you’ve written code in another programming language but
doesn’t make any assumptions about which one. We’ve tried to make the material
broadly accessible to those from a wide variety of programming backgrounds. We
don’t spend a lot of time talking about what programming *is* or how to think
about it. If you’re entirely new to programming, you would be better served by
reading a book that specifically provides an introduction to programming.

## Как использовать эту книгу

В целом, эта книга предполагает, что вы читаете ее последовательно от начала до конца. Более поздние главы основываются на концепциях предыдущих. Иногда более ранние главы могут не углубляться в детали темы; мы обычно возвращаемся к теме в следующей главе.

You’ll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of Rust. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. Chapters 2, 12, and 20 are project chapters; the rest are concept chapters.

Chapter 1 explains how to install Rust, how to write a Hello, world! program,
and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a
hands-on introduction to the Rust language. Here we cover concepts at a high
level, and later chapters will provide additional detail. If you want to get
your hands dirty right away, Chapter 2 is the place for that. At first, you
might even want to skip Chapter 3, which covers Rust features similar to those
of other programming languages, and head straight to Chapter 4 to learn about
Rust’s ownership system. However, if you’re a particularly meticulous learner
who prefers to learn every detail before moving on to the next, you might want
to skip Chapter 2 and go straight to Chapter 3, returning to Chapter 2 when
you’d like to work on a project applying the details you’ve learned.

Глава 5 описывает структуры и методы, а глава 6 охватывает перечисления, выражения `match`, и конструирукции управления потоком `if let`. Вы будете использовать структуры и перечисления для создания пользовательских типов в Rust.

In Chapter 7, you’ll learn about Rust’s module system and about privacy rules
for organizing your code and its public Application Programming Interface
(API). Chapter 8 discusses some common collection data structures that the
standard library provides, such as vectors, strings, and hash maps. Chapter 9
explores Rust’s error-handling philosophy and techniques.

Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.

Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.

В главе 16 мы рассмотрим различные модели параллельного программирования и поговорим о том, как Rust помогает вам безбоязненно программировать в нескольких потоках.
Глава 17 рассказывает о том, как идиомы Rust сравниваются с принципами объектно-ориентированного программирования, с которыми вы можете быть знакомы.

Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.

В главе 20 мы завершим проект, в котором мы реализуем низкоуровневый многопоточный веб-сервер!

Finally, some appendixes contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions.

Нет способа читать эту книгу неправильно: если вы хотите пропустить что-то и пройти вперед, сделайте это! Возможно, вам придется вернуться к предыдущим главам, если у вас появятся какие-либо затруднения. Делайте так, как считаете удобным для себя.

<span id="ferris"></span>

An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:

Ferris | Пояснения
--- | ---
<img src="img/ferris/does_not_compile.svg" class="ferris-explain"> | Этот код не компилируется!
<img src="img/ferris/panics.svg" class="ferris-explain"> | Этот код вызывает состояние "panic"!
<img src="img/ferris/unsafe.svg" class="ferris-explain"> | Этот блок кода содержит небезопасный код.
<img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"> | Этот код ведет себя не так, как предполагается.

В большинстве случаев мы приведем вас к правильной версии любого кода, который не компилируется.

## Исходный код

Файлы с исходным кодом, используемым в этой книге, можно найти на [GitHub](https://github.com/rust-lang/book/tree/master/src) .
