## Отправка запросов потокам с помощью потоков

Неполадка, которая у нас имеется в текущей выполнения следующая - наше замыкание
не делает полезной работы.

We’ve been working around the problem that we get the actual closure we want to
execute in the `execute` method, but it feels like we need to know the actual
closures when we create the `ThreadPool`.

Итак, мы хотим чтобы образец данных `Worker` создавал бы задачи, который `ThreadPool`
выполнял бы в потоке.

В главе 16 мы изучали потоки. потоки отличный способ общения между потоками и этой возможности подойдет для решения нашей задачи. Поток работает, как цепочка задач и функция `execute` будет отправлять задания из образца данных `ThreadPool` в `Worker`.

Распорядок работы:

1. `ThreadPool` будет создавать поток и будет находиться на стороне отправки.
2. Каждый `Worker` будет находится на стороне принимающей стороне.
3. Новая стопка `Job` будет содержать замыкание, которое мы хотим отправить в поток.
4. Способ `execute` стопки `ThreadPool` будет отправлять задание, которые мы хотим отослать.
5. В потоке, образец данных `Worker` в круговороте получает из потока и использует замыкания.

Приступим к созданию потока в функции `ThreadPool::new` и содержащуюся на отправляющей
стороне образец данных `ThreadPool`, как показано в рукописи 201-16. `Job` является видом данных, который мы будем отправлять в поток. Это рукопись стопки, которая пока ничего не содержит:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
// ...snip...
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // ...snip...
    pub fn new(размер: usize) -> ThreadPool {
        assert!(размер > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(размер);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // ...snip...
}
#
# struct Worker {
#     id: uразмер,
#     thread: thread::JoinHandle<()>,
# }
#
# impl Worker {
#     fn new(id: usize) -> Worker {
#         let thread = thread::spawn(|| {});
#
#         Worker {
#             id,
#             thread,
#         }
#     }
# }
```

<span class="caption">рукопись 20-16: изменение `ThreadPool`. Добавление возможности
хранения отправленной сведений в поток, которая отправляет образцы `Job`</span>

В функции `ThreadPool::new` мы создаём новый поток и затем отправляем данные. Эта рукопись собираются (хотя и c предупреждениями).

Попробуем передать принимающий конец потока каждому работнику (worker) в время их
создания. Мы знаем, что хотим использовать принимающий поток в потоке, который
появляются у рабочих, поэтому мы будем ссылаться на `receiver` при закрытии.
Эта рукопись 20-17 пока не будет собираться:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl ThreadPool {
    // ...snip...
    pub fn new(размер: usize) -> ThreadPool {
        assert!(размер > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(размер);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // ...snip...
}

// ...snip...

impl Worker {
    fn new(id: uразмер, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">рукопись 20-17: Передача принимающего конца потока
рабочим в образец данных `Worker`</span>

Это простые и очевидные изменения/

Проверяем. Получаем ошибку сборки:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0382]: use of moved значение: `receiver`
  --> src/lib.rs:27:42
   |
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here in
   previous iteration of loop
   |
   = note: move occurs because `receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
```

Т.к. мы пытаемся отправить `receiver` в несколько образцов `Worker`. Вспомнив
источник Главы 16, где выполнение потока предоставляла множество отправителей
и одного получателя, мы не можем просто удваивать получающую часть потока для решения сбоев.

Для решения межпотокового взаимодействия будем использовать умный указатель
`Arc<Mutex<T>>`. Данный указатель позволяет множеству образцов иметь получателя и
`Mutex` будет отслеживать монопольный доступ к задаче. Рукопись 201-18:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// ...snip...

# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# struct Job;
#
impl ThreadPool {
    // ...snip...
    pub fn new(размер: usize) -> ThreadPool {
        assert!(размер > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(размер);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // ...snip...
}
# struct Worker {
#     id: uразмер,
#     thread: thread::JoinHandle<()>,
# }
#
impl Worker {
    fn new(id: uразмер, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // ...snip...
#         let thread = thread::spawn(|| {
#            receiver;
#         });
#
#         Worker {
#             id,
#             thread,
#         }
    }
}
```

<span class="caption">рукопись 20-18: разделение получающую часть потока между образцами данных используя `Arc` и `Mutex`</span>

Рукопись будет собираться.

Теперь выполняем способ `execute` в `ThreadPool`. Мы также изменим устройство `Job`.
Вместо того, чтобы быть устройством - сделаем её псевдонимом сложного вида данных:

<span class="filename">Filename: src/lib.rs</span>

```rust
// ...snip...
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# use std::sync::mpsc;
# struct Worker {}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // ...snip...

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// ...snip...
```

<span class="caption">рукопись 20-19: создание вида данных `Job`, как `Box`, который
содержит замыкание, далее отправляем задание в поток</span>

После создания нового образца данных `Job`, используя способ `execute` мы отправляем
задание в поток.

Далее мы направим задание в `thread::spawn`. Нам нужно использовать бесконечный круговорот
для отслеживания задания внутри:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
// ...snip...

impl Worker {
    fn new(id: uразмер, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">рукопись 20-20: получение и выполнение заданий в круговороте, в потоке</span>

Здесь мы сначала вызываем `lock` в `receiver` для получения взаимного исключения, затем
`unwrap`. Приобретение запрета может не сработать, если взаимное исключение находится в
состояние, называемое *отравленным* (*poisoned*), которое может произойти, если
какая-то другая нить вызвал сбой удерживая замок и не освобождает его.

К сожалению, мы получим ошибку при сборке этой рукописи
Theoretically, this code should compile. Unfortunately, the Ржавчина compiler isn’t
perfect yet, and we get this error:

```text
error[E0161]: cannot move a value of type std::ops::FnOnce() +
std::marker::Send: the size of std::ops::FnOnce() + std::marker::Send cannot be
statically determined
  --> src/lib.rs:63:17
   |
63 |                 (*job)();
   |                 ^^^^^^
```

Ошибку трудно понять, т.к. неполадка сложная.

This error is fairly cryptic, and that’s because the problem is fairly cryptic.
In order to call a `FnOnce` closure that is stored in a `Box<T>` (which is what
our `Job` type alias is), the closure needs to be able to move itself out of
the `Box<T>` since when we call the closure, it takes ownership of `self`. In
general, moving a value out of a `Box<T>` isn’t allowed since Ржавчина doesn’t know
how big the value inside the `Box<T>` is going to be; recall in Chapter 15 that
we used `Box<T>` precisely because we had something of an unknown size that we
wanted to store in a `Box<T>` to get a value of a known size.

We saw in Chapter 17, Listing 17-15 that we can write methods that use the
syntax `self: Box<Self>` so that the method takes ownership of a `Self` value
that is stored in a `Box<T>`. That’s what we want to do here, but unfortunately
the part of Ржавчина that implements what happens when we call a closure isn’t
implemented using `self: Box<Self>`. So Ржавчина doesn’t yet understand that it
could use `self: Box<Self>` in this situation in order to take ownership of the
closure and move the closure out of the `Box<T>`.

In the future, the code in Listing 20-20 should work just fine. Ржавчина is still a
work in progress with places that the compiler could be improved. There are
people just like you working to fix this and other issues! Once you’ve finished
the book, we would love for you to join in.

But for now, let’s work around this problem. Luckily, there’s a trick that
involves telling Ржавчина explicitly that we’re in a case where we can take
ownership of the value inside the `Box<T>` using `self: Box<Self>`, and once we
have ownership of the closure, we can call it. This involves defining a new
trait that has a method `call_box` that uses `self: Box<Self>` in its
signature, defining that trait for any type that implements `FnOnce()`,
changing our type alias to use the new trait, and changing `Worker` to use the
`call_box` method. These changes are shown in Listing 20-21:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

// ...snip...

impl Worker {
    fn new(id: uразмер, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">рукопись 20-21: добавление сущности `FnBox` для устранения ограничений
`Box<FnOnce()>`</span>

First, we create a new trait named `FnBox`. This trait has one method,
`call_box`, similar to the `call` methods on the other `Fn*` traits, except
this method takes `self: Box<Self>` in order to take ownership of `self` and
move the value out of the `Box<T>`.

Next, we implement the `FnBox` trait for any type `F` that implements the
`FnOnce()` trait. Effectively, this means that any `FnOnce()` closures can use
our `call_box` method. The implementation of `call_box` uses `(*self)()` to
move the closure out of the `Box<T>` and call the closure.

Instead of `FnOnce()`, we now want our `Job` type alias to be a `Box` of
anything that implements our new trait `FnBox`. This will allow us to use
`call_box` in `Worker` when we get a `Job` value. Because we implemented the
`FnBox` trait for any `FnOnce()` closure, we don’t have to change anything
about the actual values we’re sending down the channel.

Finally, in the closure run in the thread in `Worker::new`, we use `call_box`
instead of invoking the closure directly. Now Ржавчина is able to understand that
what we want to do is fine.

This is a very sneaky, complicated trick. Don’t worry too much if it doesn’t
make perfect sense; someday, it will be completely unnecessary.

With this trick, our thread pool is in a working state! Give it a `cargo run`,
and make some requests:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never used: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `id`
  --> src/lib.rs:61:5
   |
61 |     id: uразмер,
   |     ^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: field is never used: `thread`
  --> src/lib.rs:62:5
   |
62 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running `target/debug/hello`
     Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

Success! We now have a thread pool executing connections asynchronously. We
never create more than four threads, so our system won’t get overloaded if the
server gets a lot of requests. If we make a request to `/sleep`, the server
will be able to serve other requests by having another thread run them.

What about those warnings, though? Don’t we use the `workers`, `id`, and
`thread` fields? Well, right now, we’re using all three of these fields to hold
onto some data, but we don’t actually *do* anything with the data once we’ve
set up the thread pool and started running the code that sends jobs down the
channel to the threads. If we didn’t hold onto these values, though, they’d go
out of scope: for example, if we didn’t return the `Vec<Worker>` value as part
of the `ThreadPool`, the vector would get cleaned up at the end of
`ThreadPool::new`.

So are these warnings wrong? In one sense yes, the warnings are wrong, since we
are using the fields to store data we need to keep around. In another sense,
no, the warnings aren’t wrong, and they’re telling us that we’ve forgotten to
do something: we never do anything to clean up our thread pool once it’s done
being used, we just use <span class="keystroke">ctrl-C</span> to stop the
program and let the operating system clean up after us. Let’s implement a
graceful shutdown that cleans up everything we’ve created instead.
