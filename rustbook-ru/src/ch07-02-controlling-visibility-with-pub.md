## Управление доступом с помощью ключевого слова `pub`

Мы исправили ошибки связанные с распределением кода. Но остались сбоев с использованием
кода (функции не используются):

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Почему имеют место эти ошибки? Библиотеки существуют для того, чтобы ими пользовались
пользователи. Поэтому важно, чтобы сбоев с доступом к возможности. были решены.

Для того, чтобы понять почему существуют такие ошибки, и как важно их устранить,
попробуем воспользоваться возможностями кода извне. Для этого создадим выполняемый
дело в этой же папки. Создадим файл *src/main.rs*, который содержит:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

Мы сейчас использовали приказ `extern crate` для того чтобы использовать возможности
библиотеки `communicator` в нашей новой программе. Cargo использует файл *src/main.rs*,
как точку доступа для двоичной программы, в то время, как *src/lib.rs* используется
для библиотеки. Такая создание кода довольно-таки обычна. Большинство кода
находится в библиотеках, а двоичный файл просто использует эту библиотеку. Итогом
такой архитектуры является возможность другим программам также использовать возможности
библиотеки.

Со стороны стороннего кода все что находится в библиотеке имеет пространство имён
`communicator` (имя библиотеки) верхнего уровня. Всё остальное, это подчиненные звенья.

Также, обратите внимание, что когда мы используем внешние библиотеки с подзвенами,
приказ `extern crate` начинает искать звенья с верхнего уровня.

В нашей программе двоичный файл вызывает библиотечную функцию `connect` из
звена `client`. Но при сборки этого кода получим сообщении об ошибке:

```text
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Это сообщение говорит нам о том, что звено `client` закрытый.
Т.к. код в звене закрытый по умолчанию и не используется внутри библиотеки - на
это надо обратить внимание, т.к. это явная ошибка в согласования кода.

После определения функции `client::connect`, как доступной (`pub`), не только сообщение
об ошибке исчезнет, но и пропадёт сообщение о том, что код не используется.
Создания доступного кода в Ржавчина даёт возможность его использования вне библиотеки.
Когда какой-либо кода помечается как `pub`, сборщик больше не сообщает об неиспользованном
коде, если даже он в действительности не используется.

### Сделать функции доступными

Для того, чтобы сделать что-либо доступным извне, необходимо добавить ключевое слово
`pub` в начало декларирования элемента кода. Для исправления ошибки, мы добавим
этот определетель доступа перед определением имени звена:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

mod network;
```

Ключевое слово `pub` перед `mod`. Сборка. Сообщение об ошибке:

```text
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Сделаем исправления в определении функции в файле *src/client.rs*:

<span class="filename">Filename: src/client.rs</span>

```rust
pub fn connect() {
}
```

Снова выполним `cargo clean && cargo build`:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Код собрался и предостережения о функции `client::connect` уже нет!

Вам решать, что делать с неиспользованным кодом, то ли открыть к нему доступ, то ли
удалить.

В данном случае мы хотим, чтобы эти функции были доступны. Поэтому добавим `pub`
там, где это необходимо:

<span class="filename">Filename: src/network/mod.rs</span>

```rust,ignore
pub fn connect() {
}

mod server;
```

Соберем и проанализируем ошибки:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | pub fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Для того, чтобы открытые функции звена были доступны, сам звено должен быть доступен:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;
```

Новые сообщения сборщика:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Надеюсь, что теперь исправить последний недочёт программы вам не составит труда.

### Правила доступа


1. Если элемент открытый, он должен быть открытый и все родительские звенья тоже.
2. Если элемент закрытый, он может быть доступен только из родительского звена и из любых подчиненных звеньев.

### Примеры

Создадим новый дело библиотеки secret 7-5 *src/lib.rs*:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

<span class="caption">Код 7-5: Примеры открытых и закрытый функций с ошибками</span>

Перед сборкой кода, попробуйте догадаться, где будет ошибка. Убедитесь в этом
с помощью сборки. Исправьте ошибки в коде!

#### Рассмотрим ошибки

Функция `try_me` находится на верхнем уровне звена нашего дела. Звено
`outermost` закрытый, но к его функциям может быть доступ, т.к. звено верхнего
уровня.

Вызов остальных двух функций вызовет ошибку, т.к. не применяются правила видимости.
Пожалуйста, исправьте ошибку!

#### Исправление ошибок

Пожалуйста, попытайтесь поэкспериментировать с доступом к функциям  и посмотрите
на описания ошибок!

Далее, мы поговорим об использовании ключевого слова `use`.
