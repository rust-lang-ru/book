## Создание внешней оболочки объединения потоков

Давайте поговорим о том, как должно выглядеть объединение. Составители часто находят что при попытке создать некоторую рукопись, найдя сначала конечного потребителя, внешнюю оболочку можно лучше понять - как лучше выполнить отдельную вычисляемую часть. Напишите API рукописи, которое будет внутренне выстроено таким образом, чтобы его было удобно вызывать, а затем запустите способ этого устройства, а не наоборот.

Подобно тому, как мы использовали Test Driven Development в деле в главе 12, здесь мы собираемся использовать Compiler Driven Development. Мы собираемся написать рукопись, которая вызывает функции, которые мы хотели бы иметь. Ошибки сборки будут направлять нашу дальнейшую разработку

### Устройства рукописи при использовании `thread::spawn`

Первое, мы рассмотрим рукопись, которую нам нужно выполнить для создания нового потока. Это не будет окончательным решение, т.к. существует вероятная неполадка (создание множества потоков), о которой мы говорили ранее. В рукописи 20-11 показаны изменения в функции `main`, которые необходимы для создания нового потока в круговороте
`for`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">рукопись 20-11: Создание нового потока для каждого соединения с конечным потребителем</span>

Как мы узнали в главе 16, `thread::spawn` создаст новый поток, а затем запустит
рукопись в замыкании. Если вы запустите эту рукопись и загрузите `/sleep` и затем `/` в
двух вкладках обозревателя, вы действительно увидите, что запрос `/` не будет
дождаться окончания `/sleep`. Но, как мы уже говорили, это в конечном итоге
будет избыточно расходовать мощности системы, так как мы создаем новые потоки
без ограничений.

### Выполнение подобного внешней оболочки с помощью `ThreadPool`

Мы хотим, чтобы объединение потоков работал похожим образом. В рукописи 20-12 заменим
предыдущее решения использование вида данных `ThreadPool`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
# struct ThreadPool;
# impl ThreadPool {
#    fn new(size: u32) -> ThreadPool { ThreadPool }
#    fn execute<F>(&self, f: F)
#        where F: FnOnce() + Send + 'static {}
# }
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">рукопись 20-12: использование `ThreadPool` без выполнения</span>

Мы используем `ThreadPool::new` для создания нового объединения с изменяемым количеством
потоков (в данном случае 4). Далее в круговороте `for` мы выполняем `pool.execute` также
как мы выполняли `thread::spawn`.

### Использование Compiler Driven Development для выполнения рабочего рукописи

Давайте попробуем собрать данная рукопись. Мы получим следующие ошибки:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module
   `ThreadPool`

error: aborting due to previous error
```

Отлично! Нам нужен `ThreadPool`. Давайте вернёмся к дополнению из двоичного файла.
Использование `ThreadPool` будет независимой от работы сетевого-отдельного вычислителя. После того, как библиотека выполняющая работу объединения потоков будет написана, мы сможем использовать её в любых выполнениех.

Итак, дополнение будет содержать файл *src/lib.rs*  с простыми определением вида данных `ThreadPool`, которую мы сейчас можем иметь:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;
```

Далее, мы создаём новую папку *src/bin* и перемещаем двоичное дополнение  *src/main.rs*
в *src/bin/main.rs*. Это сделает библиотечное дополнение основным в папке *hello*.
Это перемещение не повлияет на порядок запуска `cargo run` двоичного файла. После
перемещения файла *main.rs* внесите в самом верху писания программы изменения,
описав подключение библиотеки `hello` и её содержания в область программы
*src/bin/main.rs*:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;
```

Далее, попытайтесь теперь проверить соблюдение правил нашей рукописи, получил следующие указания сборщика для нас:

```text
$ cargo check --bins
   Compiling hello v0.1.0 (file:///projects/hello)
error: no associated item named `new` found for type `hello::ThreadPool` in the
current scope
  --> src\main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^
   |
```

Отлично! Следующим нашим действием будет выполнение функции `new` для вида данных `ThreadPool`. Также мы знаем, что функции `new` потребуется одно свойство, который может принять знание `4` в качестве переменной. Также эта функция должна возвращать
образец вида данных `ThreadPool`. Давайте выполняем такую функцию, которая будет иметь все эти свойства:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        ThreadPool
    }
}
```

Му установили `u32` в качестве вида данных входящего свойства переменной `size`, т.к. отрицательные значения не имеют смысла. Запустим проверку узнаем наше следующее следует сделать следующее действие:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

error: no method named `execute` found for type `hello::ThreadPool` in the
current scope
  --> src/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

Отлично. Предостережение и ошибка. Пока пренебрегаем предостережение. Исправим ошибку. Выполняем способ `execute`. Если вы помните главу 13, мы можем использовать замыкание в качестве свойства, как в трёх различных сущностях: `Fn`, `FnMut` и `FnOnce`.
Какую же сущность нам лучше использовать? Т.к. мы должны использовать что-то вроде `thread::spawn` мы можем посмотреть пособие:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; `T` is related to the return value and
we’re not concerned with that. Given that `spawn` uses `FnOnce` as the trait
bound on `F`, it’s probably what we want as well, since we’ll eventually be
passing the argument we get in `execute` to `spawn`. We can be further
confident that `FnOnce` is the trait that we want to use since the thread for
running a request is only going to execute that request’s closure one time.

`F` also has the trait bound `Send` and the lifetime bound `'static`, which
also make sense for our situation: we need `Send` to transfer the closure from
one thread to another, and `'static` because we don’t know how long the thread
will execute. Let’s create an `execute` method on `ThreadPool` that will take a
generic parameter `F` with these bounds:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    // ...snip...

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

The `FnOnce` trait still needs the `()` after it since this `FnOnce` is
representing a closure that takes no parameters and doesn’t return a value.
Just like function definitions, the return type can be omitted from the
signature, but even if we have no parameters, we still need the parentheses.

Again, since we’re working on getting the interface compiling, we’re adding the
simplest implementation of the `execute` method, which does nothing. Let’s
check again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

warning: unused variable: `f`, #[warn(unused_variables)] on by default
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
```

Обратите внимание, что рукопись собирается. Но если вы попытаетесь запустить программу
`cargo run` вы получите ошибки, как в начала нашей Главы. Пока наша библиотека не
готова к использованию.

> О языках со строгими сборщиками говорят (как о Haskell  и Ржавчине), что если
> рукопись собирается - она работает. Очень важно понять, что это всего лишь этап,
> а не конечное решение. Наша рукопись собирается, но она пока ещё ничего не делает. Сейчас
> наступает этап написать проверки, которые бы проверили бы соблюдение правил поведения рукописи.
