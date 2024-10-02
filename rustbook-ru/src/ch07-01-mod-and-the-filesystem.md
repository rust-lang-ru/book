## `mod` и файловая система

Мы начнём создавать наш пример использования раздела. Создадим дело библиотеки рукописей.

Создадим основные разделы нашей библиотеки, которая будет предоставлять полезные 
возможности использования сетевых технологий. Назовём нашу библиотеку `communicator`.
По умолчанию Cargo создаёт библиотеки рукописей. Если при создании нового дела мы
не установим клеймо `--bin`, то будет создана библиотека:

```text
$ cargo new communicator
$ cd communicator
```

Обратите внимание, что Cargo создаёт *src/lib.rs* вместо *src/main.rs*, в котором
мы видим вот такое содержание:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```
Cargo  создаёт пустую проверку, чтобы показать как можно проверять возможности библиотеки.
Мы изучим использование связанных стопок `#[]` и `mod tests` в последующих разделах "Использование `super` для доступа к родительскому разделу" этой Главы.

Сейчас же мы не будем использовать данные возможности, поэтому просто удалим эту рукопись.

Т.к. у нас нет файла *src/main.rs*, нечего запускать на выполнение с помощью приказов
`cargo run`. В тоже время мы можем воспользоваться приказом `cargo build` для сборки
нашей библиотеки.

Мы рассмотрим различные возможности согласования рукописей нашей библиотеки.

### Определение раздела

Первым делом напишем определение раздела `network`, которое будет содержать
определение функции `connect`. Определение начинается с ключевого слова `mod`.


<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}
```

После определения раздела, внутри узорчатых скобок пишем определения функции и
все что входит в состав раздела. В нашем случае это описание функции.
Если мы хотим вызывать функцию извне раздела, мы должны явно указать это `network::connect()`.

У нас может быть множество описаний разделов в одном файле  *src/lib.rs*.
К примеру, раздел `client`, может содержать функцию `connect` 7-1:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

<span class="caption">Пример 7-1: Определение разделов `network` и `client`в файле
 *src/lib.rs*</span>

Теперь у нас есть описание двух функций, которые могут быть вызваны с помощью связанных стопок `network::connect` и `client::connect`.
У каждой из этих функций могут быть различные полезные  возможности, но у них нет
между собой никакого несоответствия имён.

В этом случае, если мы создаём библиотеку, файл *src/lib.rs* хранит точку доступа к
библиотеке.  Также мы можем создать раздел в файле  *src/main.rs*
для какой-либо двоичной программы. Очень важная особенность разделов - они могут быть вложенными. 
Это весьма удобно для разумного согласования рукописи.
Пример 7-2:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

<span class="caption"> Пример 7-2: Перемещение раздела `client` внутрь раздела `network`</span>

Теперь у нас есть две разные функции `network::connect` и `network::client::connect`.
Каждая из которых находится в своём пространстве имён.

Теперь создание нашей рукописи имеет вот такое устройство:

```text
communicator
 ├── network
 └── client
```

Пример вложенных разделов 7-2:

```text
communicator
 └── network
     └── client
```

Разумное создание рукописи зависит от ваших задач.

### Размещение разделов по нескольким файлам

Состоящее из разделов устройство похоже на файловую систему. Мы можем использовать состоящую из разделов
систему для хранения рукописи в разных файлах. Рассмотрим пример 7-3:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<span class="caption">Пример 7-3: Разделы `client`, `network` и`network::server`,
все они находятся в *src/lib.rs*</span>

Архитектура разделов *src/lib.rs*:

```text
communicator
 ├── client
 └── network
     └── server
```

Если разделы имеют множество функций и эти функции длинные, было бы удобно разделить такую рукопись на несколько файлов.

Сначала заменим рукопись раздела `client` на объявление раздела:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

Тут мы видим объявление раздела. Этим мы сообщаем, что в другом месте есть определение раздела `client`:

```rust,ignore
mod client {
    // contents of client.rs
}
```

Теперь создадим файл *client.rs* в папки *src*:

<span class="filename">Filename: src/client.rs</span>

```rust
fn connect() {
}
```
Обратите внимание, что вам не надо объявлять раздел, т.к. вы уже объявили
его в файле *src/lib.rs*. Файл *src/client.rs* содержит составляющие раздела `client`. Если же
вы и здесь напишите объявление раздела `mod client`, то это будет значит, что внутри раздела `client` есть раздел `client`!

По умолчанию, сборщик сначала исследует содержание файла *src/lib.rs*. Если
есть необходимость добавить несколько файлов в дело, необходимо сообщить об этом
в файле *src/lib.rs*. Именно поэтому, раздел `client` надо определить в файле *src/lib.rs*
и не надо делать этого в файле *src/client.rs*.

Сборка дела пройдет успешно:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:4:5
  |
4 |     fn connect() {
  |     ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:8:9
  |
8 |         fn connect() {
  |         ^
```

Эти сообщения сообщают нам, что наши функции нигде не используются. Пренебрегаем
ими до разделы "Управление доступом с помощью ключевого слова `pub`".

Теперь перенесём раздел `network` в свой файл:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network;
```

Далее, создадим файл *src/network.rs* и введём в него следующую рукопись:

<span class="filename">Filename: src/network.rs</span>

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```
Обратите внимание, что у нас есть описание разделов в файле, т.к. у нас всё еще есть
вложенность разделов.

Выполним приказы `cargo clean`, а потом `cargo build`. Всё в порядке! Отлично!
Теперь осталось создать файл только для ещё одного раздела. Для этого создадим
описание подчиненного раздела в файле *src/network.rs* `mod server;`:

<span class="filename">Filename: src/network.rs</span>

```rust,ignore
fn connect() {
}

mod server;
```

Далее создадим файл *src/server.rs* и добавим в него содержание:

<span class="filename">Filename: src/server.rs</span>

```rust
fn connect() {
}
```

Выполним приказы `cargo clean`, а потом `cargo build`. Получим сообщение об ошибке 7-4:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

<span class="caption">Рукопись 7-4: Ошибка при переносе рукописи вложенного раздела `server`
в файл *src/server.rs*</span>

Сборщик предлагает решение:

```text
note: maybe move this module `network` to its own directory via
`network/mod.rs`
```
Вместо того, чтобы создавать файл подобно предыдущему, сделаем следующее:

1. Создадим *папку* с именем *network* (это имя нашего родительского раздела).
2. Перенесём файл *src/network.rs* в эту новую папку и переименуем файл в *src/network/mod.rs*.
3. Далее перенесём файл *src/server.rs* в папку *network*.

 Для подчиненных разделов проделаем тоже самое.

### Правила состоящей из разделов файловой системы

Список правил:

* Если раздел `foo` не имеет подчиненных разделов, вы можете сохранить рукопись раздела в
файл *foo.rs*.
* Если раздел `foo` имеет подраздел, вы должны перенести рукопись раздела в файл *foo/mod.rs*

Это правило применяется рекурсивно. Если раздел с именем `foo` имеет подраздел
`bar` и `bar` не имеет подразделов, то у вас получится вот такое устройство
в папке *src*:

```text
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

Разделы должны быть определены в своих файлах используя ключевое слово `mod`.

Далее, мы поговорим о изменителе доступа `pub` и устраним сообщения о неполадках
в рукописи.
