## Создание однопоточного веб-сервера

Начнем с однопоточного веб-сервера. Перед тем, как начать, давайте рассмотрим краткий обзор протоколов, задействованных в создании веб-серверов. Детальное описание этих протоколов выходит за рамки этой книги, но краткий обзор даст вам необходимую информацию.

Два основных протокола, задействованных в веб-серверах, - это *протокол передачи гипертекста* *(HTTP)* и *протокол управления передачей* *(TCP)*. Оба протокола являются протоколами типа *запрос-ответ*, что означает, что *клиент* инициирует запросы, а *сервер* слушает запросы и предоставляет ответ клиенту. Содержание этих запросов и ответов определяется протоколами.

TCP - это протокол нижнего уровня, который описывает детали того, как информация передаётся от одного сервера к другому, но не определяет, что это за информация. HTTP строится поверх TCP, определяя содержимое запросов и ответов. Технически возможно использовать HTTP с другими протоколами, но в подавляющем большинстве случаев HTTP отправляет свои данные поверх TCP. Мы будем работать с необработанными байтами в TCP и запросами и ответами в HTTP.

### Прослушивание TCP соединения

Нашему веб-серверу необходимо прослушивать TCP-соединение, так что это первая часть, над которой мы будем работать. Стандартная библиотека предлагает для этого модуль `std::net`. Сделаем новый проект обычным способом:

```console
$ cargo new hello
      Created binary (application) `hello` project
$ cd hello
```

Теперь введите код из Листинга 20-1 в *src/main.rs*, чтобы начать. Этот код будет использовать адрес `127.0.0.1:7878` для входящих TCP-потоков. Когда он получит входящее соединение, он напечатает `Connection established!`,

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-01/src/main.rs}}
```

<span class="caption">Listing 20-1: Listening for incoming streams and printing a message when we receive a stream</span>

Используя `TcpListener`, мы можем прослушивать TCP-соединения по адресу `127.0.0.1:7878`. В адресе раздел перед двоеточием - это IP-адрес, представляющий локальный компьютер (он одинаков на всех компьютерах и не представляет конкретно компьютер авторов), а `7878` - это порт. Мы выбрали этот порт по двум причинам: HTTP может быть поднят на этом порту, и 7878 - это *rust*, набранный на телефоне.

The `bind` function in this scenario works like the `new` function in that it will return a new `TcpListener` instance. The reason the function is called `bind` is that in networking, connecting to a port to listen to is known as “binding to a port.”

The `bind` function returns a `Result<T, E>`, which indicates that binding might fail. For example, connecting to port 80 requires administrator privileges (nonadministrators can listen only on ports higher than 1023), so if we tried to connect to port 80 without being an administrator, binding wouldn’t work. As another example, binding wouldn’t work if we ran two instances of our program and so had two programs listening to the same port. Because we’re writing a basic server just for learning purposes, we won’t worry about handling these kinds of errors; instead, we use `unwrap` to stop the program if errors happen.

The `incoming` method on `TcpListener` returns an iterator that gives us a sequence of streams (more specifically, streams of type `TcpStream`). A single *stream* represents an open connection between the client and the server. A *connection* is the name for the full request and response process in which a client connects to the server, the server generates a response, and the server closes the connection. As such, `TcpStream` will read from itself to see what the client sent and then allow us to write our response to the stream. Overall, this `for` loop will process each connection in turn and produce a series of streams for us to handle.

For now, our handling of the stream consists of calling `unwrap` to terminate our program if the stream has any errors; if there aren’t any errors, the program prints a message. We’ll add more functionality for the success case in the next listing. The reason we might receive errors from the `incoming` method when a client connects to the server is that we’re not actually iterating over connections. Instead, we’re iterating over *connection attempts*. The connection might not be successful for a number of reasons, many of them operating system specific. For example, many operating systems have a limit to the number of simultaneous open connections they can support; new connection attempts beyond that number will produce an error until some of the open connections are closed.

Попробуем запустить этот код! Вызовите `cargo run` в терминале, а затем загрузите *127.0.0.1:7878* в веб-браузере. В браузере должно отображаться сообщение об ошибке, например «Connection reset», поскольку сервер в настоящее время не отправляет обратно никаких данных. Но когда вы посмотрите на свой терминал, вы должны увидеть несколько сообщений, которые были напечатаны, когда браузер подключался к серверу!

```text
     Running `target/debug/hello`
 Connection established!
 Connection established!
 Connection established!
```

Sometimes, you’ll see multiple messages printed for one browser request; the reason might be that the browser is making a request for the page as well as a request for other resources, like the *favicon.ico* icon that appears in the browser tab.

Также может быть, что браузер пытается подключиться к серверу несколько раз, потому что сервер не отвечает. Когда `stream` выходит из области видимости и отбрасывается в конце цикла, соединение закрывается как часть реализации `drop`. Браузеры иногда обрабатывают закрытые соединения, повторяя попытки, потому что проблема может быть временной. Важным фактором является то, что мы успешно получили дескриптор TCP-соединения!

Remember to stop the program by pressing <span class="keystroke">ctrl-c</span> when you’re done running a particular version of the code. Then restart `cargo run` after you’ve made each set of code changes to make sure you’re running the newest code.

### Чтение запросов

Реализуем функционал чтения запроса из браузера! Чтобы разделить части, связанные с получением соединения и последующим действием с ним, мы запустим новую функцию для обработки соединения. В этой новой функции `handle_connection` мы будем читать данные из потока TCP и распечатывать их, чтобы мы могли видеть данные, отправленные из браузера. Измените код, чтобы он выглядел как в листинге 20-2.

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-02/src/main.rs}}
```

<span class="caption">Listing 20-2: Reading from the <code>TcpStream</code> and printing the data</span>

Мы добавляем `std::io::prelude` в область видимости, чтобы получить доступ к определённым свойствам, которые позволяют нам читать и писать в поток. В цикле `for` функции `main` вместо вывода сообщения о том, что мы установили соединение, мы теперь вызываем новую функцию `handle_connection` и передаём ей `stream`.

In the `handle_connection` function, we’ve made the `stream` parameter mutable. The reason is that the `TcpStream` instance keeps track of what data it returns to us internally. It might read more data than we asked for and save that data for the next time we ask for data. It therefore needs to be `mut` because its internal state might change; usually, we think of “reading” as not needing mutation, but in this case we need the `mut` keyword.

Далее нам нужно фактически прочитать данные из потока. Мы делаем это в два этапа: во-первых, мы объявляем `buffer` в стеке для хранения считываемых данных. Мы сделали буфер размером 1024 байта, что достаточно для хранения данных базового запроса и достаточно для наших целей в этой главе. Если бы мы хотели обрабатывать запросы произвольного размера, управление буфером должно было бы быть более сложным; пока делаем проще. Мы передаём буфер в `stream.read` , который считывает байты из `TcpStream` и помещает их в буфер.

Second, we convert the bytes in the buffer to a string and print that string. The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String` from it. The “lossy” part of the name indicates the behavior of this function when it sees an invalid UTF-8 sequence: it will replace the invalid sequence with `�`, the `U+FFFD REPLACEMENT CHARACTER`. You might see replacement characters for characters in the buffer that aren’t filled by request data.

Попробуем этот код! Запустите программу и снова сделайте запрос в веб-браузере. Обратите внимание, что мы по-прежнему будем получать в браузере страницу с ошибкой, но вывод нашей программы в терминале теперь будет выглядеть примерно так:

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

В зависимости от вашего браузера результат может немного отличаться. Теперь, когда мы печатаем данные запроса, мы можем понять, почему мы получаем несколько подключений из одного запроса браузера, посмотрев на путь после `Request: GET` . Если все повторяющиеся соединения запрашивают */* , мы знаем, что браузер пытается получить */* повторно, потому что он не получает ответа от нашей программы.

Давайте разберём эти данные запроса, чтобы понять, что браузер запрашивает у нашей программы.

### Пристальный взгляд на HTTP запрос

HTTP - это текстовый протокол и запрос имеет следующий формат:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is the *request line* that holds information about what the client is requesting. The first part of the request line indicates the *method* being used, such as `GET` or `POST`, which describes how the client is making this request. Our client used a `GET` request.

Следующая часть строки запроса - это */*, которая указывает *унифицированный идентификатор* *ресурса (URI),* который запрашивает клиент: URI почти, но не совсем то же самое, что и *унифицированный указатель ресурса* *(URL)*. Разница между URI и URL-адресами не важна для наших целей в этой главе, но спецификация HTTP использует термин URI, поэтому мы можем просто мысленно заменить URL-адрес здесь.

Последняя часть - это версия HTTP, которую использует клиент, а затем строка запроса заканчивается *последовательностью CRLF* . (CRLF обозначает *возврат каретки* и *перевод строки* , что является термином из дней пишущих машинок!) Последовательность CRLF также может быть записана как `\r\n` , где `\r` - возврат каретки, а `\n` - перевод строки. Последовательность CRLF отделяет строку запроса от остальных данных запроса. Обратите внимание, что при печати CRLF мы видим начало новой строки, а не `\r\n` .

Глядя на данные строки запроса, которые мы получили от запуска нашей программы, мы видим, что `GET` - это метод, */* - это URI запроса, а `HTTP/1.1` - это версия.

После строки запроса оставшиеся строки, начиная с `Host:` далее, являются заголовками. `GET` запросы не имеют тела.

Попробуйте сделать запрос из другого браузера или запросить другой адрес, например *127.0.0.1:7878/test* , чтобы увидеть, как изменяются данные запроса.

Теперь, когда мы знаем, что запрашивает браузер, давайте отправим обратно в ответ некоторые данные!

### Написание ответа

Now we’ll implement sending data in response to a client request. Responses have the following format:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is a *status line* that contains the HTTP version used in the response, a numeric status code that summarizes the result of the request, and a reason phrase that provides a text description of the status code. After the CRLF sequence are any headers, another CRLF sequence, and the body of the response.

Here is an example response that uses HTTP version 1.1, has a status code of 200, an OK reason phrase, no headers, and no body:

```text
HTTP/1.1 200 OK\r\n\r\n
```

Код состояния 200 - это стандартный успешный ответ. Текст представляет собой крошечный успешный HTTP-ответ. Давайте запишем это в поток как наш ответ на успешный запрос! Из функции `handle_connection` удалите `println!` который печатал данные запроса и заменял их кодом из Листинга 20-3.

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-03/src/main.rs:here}}
```

<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to the stream</span>

Первая новая строка определяет переменную `response` которая содержит данные сообщения об успешном выполнении. Затем мы вызываем `as_bytes` в нашем `response` чтобы преобразовать строковые данные в байты. Метод `write` в `stream` принимает `&[u8]` и отправляет эти байты напрямую по соединению.

Because the `write` operation could fail, we use `unwrap` on any error result as before. Again, in a real application you would add error handling here. Finally, `flush` will wait and prevent the program from continuing until all the bytes are written to the connection; `TcpStream` contains an internal buffer to minimize calls to the underlying operating system.

With these changes, let’s run our code and make a request. We’re no longer printing any data to the terminal, so we won’t see any output other than the output from Cargo. When you load *127.0.0.1:7878* in a web browser, you should get a blank page instead of an error. You’ve just hand-coded an HTTP request and response!

### Возвращение реального HTML

Реализуем функционал для возврата более пустой страницы. Создайте новый файл *hello.html* в корне каталога вашего проекта, а не в каталоге *src* . Вы можете ввести любой HTML-код; В листинге 20-4 показана одна возможность.

<span class="filename">Файл: hello.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-04/hello.html}}
```

<span class="caption">Листинг 20-4. Образец HTML-файла для возврата в ответ</span>

This is a minimal HTML5 document with a heading and some text. To return this from the server when a request is received, we’ll modify `handle_connection` as shown in Listing 20-5 to read the HTML file, add it to the response as a body, and send it.

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-05/src/main.rs:here}}
```

<span class="caption">Листинг 20-5. Отправка содержимого <em>hello.html</em> в качестве тела ответа</span>

We’ve added a line at the top to bring the standard library’s filesystem module into scope. The code for reading the contents of a file to a string should look familiar; we used it in Chapter 12 when we read the contents of a file for our I/O project in Listing 12-4.

Далее мы используем `format!` чтобы добавить содержимое файла в качестве тела ответа об успешном завершении. Чтобы гарантировать действительный HTTP-ответ, мы добавляем заголовок `Content-Length` который имеет размер тела нашего ответа, в данном случае размер `hello.html` .

Run this code with `cargo run` and load *127.0.0.1:7878* in your browser; you should see your HTML rendered!

В настоящее время мы игнорируем данные запроса в `buffer` и просто безоговорочно отправляем обратно содержимое HTML-файла. Это означает, что если вы попытаетесь запросить *127.0.0.1:7878/something-else* в своём браузере, вы все равно получите тот же ответ HTML. Наш сервер очень ограничен, и это не то, что делает большинство веб-серверов. Мы хотим настроить наши ответы в зависимости от запроса и отправлять обратно HTML-файл только для правильно сформированного запроса в */* .

### Проверка запроса и выборочное возвращение ответа

Прямо сейчас наш веб-сервер вернёт HTML-код в файле независимо от того, что запросил клиент. Давайте добавим функциональность, чтобы проверять, запрашивает ли браузер */* перед возвратом HTML-файла, и возвращать ошибку, если браузер запрашивает что-либо ещё. Для этого нам нужно изменить `handle_connection` , как показано в листинге 20-6. Этот новый код проверяет содержимое полученного запроса на соответствие тому, как мы знаем, что запрос на */* выглядит как, и добавляет блоки `if` и `else` чтобы обрабатывать запросы по-разному.

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-06/src/main.rs:here}}
```

<span class="caption">Listing 20-6: Matching the request and handling requests to <em>/</em> differently from other requests</span>

First, we hardcode the data corresponding to the */* request into the `get` variable. Because we’re reading raw bytes into the buffer, we transform `get` into a byte string by adding the `b""` byte string syntax at the start of the content data. Then we check whether `buffer` starts with the bytes in `get`. If it does, it means we’ve received a well-formed request to */*, which is the success case we’ll handle in the `if` block that returns the contents of our HTML file.

Если `buffer` *не* начинается с байтов в `get` , это означает, что мы получили другой запрос. Мы добавим код в блок `else` через мгновение, чтобы ответить на все остальные запросы.

Запустите этот код сейчас и запросите *127.0.0.1:7878* ; вы должны получить HTML в *hello.html* . Если вы сделаете любой другой запрос, например *127.0.0.1:7878/something-else* , вы получите ошибку соединения, подобную той, которую вы видели при запуске кода из Листинга 20-1 и Листинга 20-2.

Now let’s add the code in Listing 20-7 to the `else` block to return a response with the status code 404, which signals that the content for the request was not found. We’ll also return some HTML for a page to render in the browser indicating the response to the end user.

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-07/src/main.rs:here}}
```

<span class="caption">Listing 20-7: Responding with status code 404 and an error page if anything other than <em>/</em> was requested</span>

Здесь наш ответ содержит строку состояния с кодом состояния 404 и фразой причины `NOT FOUND` . Мы по-прежнему не возвращаем заголовки, и телом ответа будет HTML в файле *404.html* . Вам нужно будет создать файл *404.html* рядом с *hello.html* для страницы с ошибкой; снова не стесняйтесь использовать любой HTML, какой хотите, или используйте пример HTML из Листинга 20-8.

<span class="filename">Файл: 404.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-08/404.html}}
```

<span class="caption">Листинг 20-8. Пример содержимого страницы для отправки с любым ответом 404</span>

С этими изменениями снова запустите сервер. Запрос на *127.0.0.1:7878* должен возвращать содержимое *hello.html*, и любой другой запрос, как *127.0.0.1:7878/foo*, должен возвращать сообщение об ошибке HTML от *404.html*.

### Рефакторинг

В настоящий момент блоки `if` и `else` часто повторяются: они читают файлы и записывают содержимое файлов в поток. Единственные различия - это строка состояния и имя файла. Давайте сделаем код более кратким, выделив эти различия в отдельные строки `if` и `else` которые будут назначать значения строки состояния и имени файла переменным; затем мы можем безоговорочно использовать эти переменные в коде для чтения файла и записи ответа. В листинге 20-9 показан код, полученный после замены больших блоков `if` и `else` .

<span class="filename">Файл: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-09/src/main.rs:here}}
```

<span class="caption">Listing 20-9: Refactoring the <code>if</code> and <code>else</code> blocks to contain only the code that differs between the two cases</span>

Теперь блоки `if` и `else` возвращают только соответствующие значения для строки состояния и имени файла в кортеже; Затем мы используем деструктурирование, чтобы присвоить эти два значения `status_line` и `filename` используя шаблон в операторе `let`, как обсуждалось в главе 18.

The previously duplicated code is now outside the `if` and `else` blocks and uses the `status_line` and `filename` variables. This makes it easier to see the difference between the two cases, and it means we have only one place to update the code if we want to change how the file reading and response writing work. The behavior of the code in Listing 20-9 will be the same as that in Listing 20-8.

Awesome! We now have a simple web server in approximately 40 lines of Rust code that responds to one request with a page of content and responds to all other requests with a 404 response.

Currently, our server runs in a single thread, meaning it can only serve one request at a time. Let’s examine how that can be a problem by simulating some slow requests. Then we’ll fix it so our server can handle multiple requests at once.
