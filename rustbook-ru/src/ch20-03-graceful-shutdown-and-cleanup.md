## Изящное завершение и освобождение ресурсов

Код 20-21 асинхронно отвечает на запросы с помощью использования пула потоков, как мы и хотели. Мы получаем некоторые предупреждения про `workers`, `id` и поля `thread`, которые мы не используем напрямую, что напоминает нам о том, что мы не освобождаем все ресурсы. Когда мы используем менее элегантный метод остановки основного потока клавишной комбинацией <span class="keystroke">ctrl-c</span>, все остальные потоки также немедленно останавливаются, даже если они находятся в середине обработки запроса.

Теперь мы будем реализовывать типаж `Drop` для вызова `join` у каждого потока в пуле, чтобы они могли завершить запросы над которыми они работают до закрытия. Затем мы реализуем способ сообщить потокам, что они должны перестать принимать новые запросы и завершить работу. Чтобы увидеть этот код в действии, мы изменим наш сервер так, чтобы он принимал только два запроса, прежде чем корректно завершить работу его пула потоков.

### Реализация типажа `Drop` для `ThreadPool`

Давайте начнем с реализации `Drop` у нашего пула потоков. Когда пул удаляется, все наши потоки должны объединиться (join), чтобы убедиться, что они завершают свою работу. В коде 20-23 показана первая попытка реализации `Drop`, код пока не работает.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

<span class="caption">Код 20-23: Присоединение (Joining) каждого потока, когда пул потоков выходит из области видимости</span>

Во-первых, мы проходим циклом по каждому `workers` из пула потоков. Для этого мы используем `&mut`, потому что `self` является изменяемой ссылкой и нам также нужно иметь возможность изменять экземпляр `worker`. Для каждого "работника" мы печатаем сообщение о том, что этот конкретный "работник" завершается, затем вызываем `join` у потока этого "работника". Если вызов `join` происходит с ошибкой, мы используем `unwrap`, чтобы вызвать панику в Rust и завершить не совсем красиво.

Ошибка получаемая при компиляции этого кода:

```text
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

Ошибка говорит, что мы не можем вызвать `join`, потому что у нас есть только изменяемое заимствование каждого `worker` и что `join` забирает во владение его аргумент. Чтобы решить эту проблему, нужно переместить поток из экземпляра `Worker`, который владеет `thread`, чтобы `join` мог использовать внутренний поток. Мы сделали это в коде 17-15: если вместо этого `Worker` содержит тип `Option<thread::JoinHandle<()>>`, мы можем вызвать метод `take` у `Option`, чтобы переместить значение из варианта `Some` и оставить вариант `None` на его месте. Другими словами, работающий `Worker` будет содержать вариант `Some` внутри `thread`, и когда мы хотим очистить `Worker`, мы заменяем значение варианта `Some` на вариант `None`, чтобы у `Worker` не было потока для запуска.

Итак, мы хотим обновить объявление `Worker` следующим образом:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

Теперь давайте опираться на компилятор, чтобы найти другие места, которые нужно изменить. Проверяя код, мы получаем две ошибки:

```text
error[E0599]: no method named `join` found for type
`std::option::Option<std::thread::JoinHandle<()>>` in the current scope
  --> src/lib.rs:65:27
   |
65 |             worker.thread.join().unwrap();
   |                           ^^^^

error[E0308]: mismatched types
  --> src/lib.rs:89:13
   |
89 |             thread,
   |             ^^^^^^
   |             |
   |             expected enum `std::option::Option`, found struct
   `std::thread::JoinHandle`
   |             help: try using a variant of the expected type: `Some(thread)`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`
```

Давайте обратимся ко второй ошибке, которая указывает на код в конце `Worker::new`; нам нужно обернуть значение `thread` в вариант `Some` при создании нового `Worker`. Внесите следующие изменения, чтобы исправить эту ошибку:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

Первая ошибка находится в нашей реализации `Drop`. Ранее мы упоминали, что намеревались вызвать `take` для параметра `Option`, чтобы забрать `thread` из процесса `worker`. Следующие изменения делают это:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

Как уже говорилось в главе 17, метод `take` у типа `Option` забирает значение из варианта `Some` и оставляет вариант `None` в этом месте. Мы используем `if let`, чтобы деструктурировать `Some` и получить поток; затем вызываем `join` у потока. Если поток "работника" уже `None`, мы знаем, что этот "работник" уже очистил свой поток, поэтому в этом случае ничего не происходит.

### Сигнализация потокам прекратить прослушивание получения задач

Теперь со всеми внесенными нами изменениями код компилируется без каких-либо предупреждений. Но плохая новость в том, что этот код еще не работает так, как мы этого хотим. Ключом является логика в замыканиях, запускаемых потоками экземпляров `Worker`. В данный момент мы вызываем `join`, но это не завершит потоки, потому что они работают в цикле `loop` бесконечно в поиске новой задачи (job). Если мы попытаемся удалить `ThreadPool` в текущей реализации `drop`, то основной поток навсегда заблокируется в ожидании завершения первого потока из пула.

Для решения этой проблемы, мы изменим потоки так, чтобы они прослушивали либо задачи `Job` для ее выполнения, либо сигнал, что они должны прекратить прослушивание и выйти из бесконечного цикла. Вместо отправки экземпляров задач `Job`, наш канал отправит один из этих двух вариантов перечисления.

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Job;
enum Message {
    NewJob(Job),
    Terminate,
}
```

Это перечисление `Message` будет либо вариантом `NewJob`, который внутри держит `Job` с потоком для выполнения, или это будет вариант `Terminate`, который сделает так, чтобы поток вышел из цикла и остановился.

Нам нужно настроить канал для использования значений типа `Message`, а не типа `Job` как показано в коде 20-24.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// --snip--

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<span class="caption">Код 20-24: Отправка и получение значений перечисления <code>Message</code> и выход из цикла, если <code>Worker</code> получает перечисление <code>Message::Terminate</code></span>

Чтобы встроить в код перечисление `Message`, нужно изменить `Job` на `Message` в двух местах: в объявлении `ThreadPool` и сигнатуре `Worker::new`. Метод `execute` у `ThreadPool` должен отправлять задания, завернутые в вариант `Message::NewJob`. Затем в `Worker::new`, где `Message` получен из канала, задание (job) будет обработано, если получен вариант `NewJob` и поток выйдет из цикла, если получен вариант `Terminate`.

С такими изменениями код компилируется и продолжит функционировать так же, как и после кода 20-21. Но мы получим предупреждение, потому что мы не создаем сообщения типа `Terminate`. Давайте исправим это предупреждение, изменив нашу реализацию `Drop` как в коде 20-25.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

<span class="caption">Код 20-25: Отправка <code>Message::Terminate</code> рабочим потокам перед вызовом <code>join</code> в каждом потоке</span>

Теперь мы дважды проходим по потокам "работников": один раз для отправки одного сообщения `Terminate` каждому потоку и один раз для вызова `join` у каждого рабочего потока. Если бы мы попытались отправить сообщение и сразу же выполнить `join` в этом же цикле, мы не смогли бы гарантировать, что рабочий поток в текущей итерации получит сообщение из канала.

Чтобы лучше понять, почему нужны два отдельных цикла, представьте сценарий с двумя работниками. Если бы мы использовали один цикл для перебора каждого работника, на первой итерации сообщение о прекращении работы было бы отправлено в канал и метод `join` вызывался бы у первого рабочего потока. Если этот первый поток занят обработкой запроса в данный момент, второй рабочий поток получит сообщение о завершении из канала и завершит свою работу. В главном потоке мы бы остались в ожидании завершения работы первого рабочего потока, но этого не произошло, потому что второй поток получил сообщение о завершении. Это ситуация взаимной блокировки (deadlock)!

Чтобы предотвратить такой сценарий, мы сначала помещаем все сообщения `Terminate` в канал в первом цикле; затем мы объединяем (join) для завершения все потоки во втором цикле. Каждый рабочий поток прекратит получать запросы из канала, как только получит сообщение о завершении. Таким образом, мы можем быть уверены, что если мы отправим количество завершающих сообщений равное количеству рабочих потоков то, каждый получит сообщение о завершении до вызова `join` в его потоке.

Чтобы увидеть этот код в действии, давайте изменим `main`, чтобы принимать только два запроса, прежде чем корректно завершить работу сервера как показано в коде 20-26.

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
```

<span class="caption">Код 20-26. Выключение сервера после обслуживания двух запросов с помощью выхода из цикла</span>

Вы бы не хотели, чтобы реальный веб-сервер отключался после обслуживания только двух запросов. Этот код всего лишь демонстрирует, что корректное завершение работы и освобождение ресурсов находятся в рабочем состоянии.

Метод `take` определен в типаже `Iterator` и ограничивает итерацию максимум первыми двумя элементами. `ThreadPool` выйдет из области видимости в конце `main` и будет запущена его реализация `drop`.

Запустите сервер с `cargo run` и сделайте три запроса. Третий запрос должен выдать ошибку и в терминале вы должны увидеть вывод, подобный следующему:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

Вы возможно увидите другой порядок рабочих потоков и напечатанных сообщений. Мы можем увидеть, как этот код работает по сообщениям: "работники" номер 0 и 3 получили первые два запроса, затем на третьем запросе сервер прекратил принимать соединения. Когда `ThreadPool` выходит из области видимости в конце `main`, то срабатывает его реализация типажа `Drop` и пул сообщает всем рабочим потокам прекратить выполнение. Каждый рабочий поток распечатывает сообщение, когда видит сообщение о завершении, а затем пул потоков вызывает `join`, чтобы завершить работу каждого рабочего потока.

Обратите внимание на один интересный аспект этого конкретного выполнения: `ThreadPool` отправил сообщения о завершении в канал и прежде чем какой-либо рабочий получил сообщение, мы пытались присоединить (join) "работника" с номером 0. Рабочий поток 0 еще не получил сообщение о прекращении, поэтому основной поток заблокировал ожидание потока 0 для завершения. Тем временем каждый из рабочих потоков получил сообщения об завершении. Когда рабочий поток 0 завершился, основной поток ждал окончания завершения выполнения остальных рабочих потоков. В этот момент все они получили сообщение о завершении и смогли завершиться.

Поздравления! Теперь мы завершили проект; у нас есть базовый веб-сервер, который использует пул потоков для асинхронных ответов. Мы можем выполнить корректное завершение работы сервера, который очищает все потоки в пуле.

Вот полный код для справки:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

В коде можно сделать больше! Если вы хотите продолжить совершенствование этого проекта, вот несколько идей:

- Добавьте больше документации в `ThreadPool` и его публичным методам.
- Добавьте тесты функционалу из библиотеки.
- Заменить вызовы `unwrap` на более правильную обработку ошибок.
- Используйте `ThreadPool` для выполнения некоторых других задач, помимо обслуживания веб-запросов.
- Найдите крейт для пула потоков на [crates.io](https://crates.io/) и реализуйте аналогичный веб-сервер, используя такой крейт. Затем сравните его API и надежность с пулом потоков, который мы реализовали.

## Summary

Отличная работа! Вы сделали это к концу книги! Мы хотим поблагодарить вас за то, что присоединились к нам в этом путешествии по языку Rust. Теперь вы готовы реализовать свои собственные проекты на Rust и помочь с проектами другим людям. Имейте в виду, что существует приветливое сообщество других Rust разработчиков, которые хотели бы помочь вам с любыми сложными задачами с которыми вы столкнетесь в своем Rust путешествии.
