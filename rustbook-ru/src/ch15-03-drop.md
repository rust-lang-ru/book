## Запуск кода при очистке с помощью типажа `Drop`

Другим важным типажом шаблонов умных указателей является `Drop`. Он позволяет выполнятся коду в момент окончания существования значения. Умные указатели выполняют отчистку памяти, кода элементы становятся недействительными. Т.е. типы данных могут управлять ресурсами в памяти, такими как файлы и подключения и использовать типаж `Drop` в контексте умных указателей.

В некоторых языках программирования такой функционал необходимо реализовывать программисту самостоятельно. Если вы забудете это сделать это может привести к перерасходованию памяти и  даже к аварии.

Для реализации типажа `Drop` необходимо реализовать метод `drop`, который получает в качестве параметра изменяемую ссылку на `self`.

В листинге 15-14 показана структура `CustomSmartPointer`, единственная функция которой заключается в том, что она будет печатать `Dropping CustomSmartPointer!` когда экземпляр выходит за область видимости. Этот пример демонстрирует, когда Rust выполняет функцию `drop` .

<span class="filename">Файл: src/main.rs</span>

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

<span class="caption">код 15-8: работы типажа <code>Drop</code> при реализации его структурой <code>CustomSmartPointer</code> после выхода экземпляра этой структуры из области видимости</span>

Типаж «Drop» загружается неявным образом в область действия ПО, поэтому вам не нужно явным образом его импортировать. Метод `drop` реализован. В нем вызывается макрос `println!`. Обратите внимание, что метод `drop` был вызван неявным образов в момент удаления переменной из памяти.

В `main` мы создаем два экземпляра `CustomSmartPointer` и затем печатаем `CustomSmartPointers created` . В конце `main` наши экземпляры `CustomSmartPointer` выйдут из области видимости и Rust вызовет код, который мы добавили в метод `drop`, который и напечатает наше окончательное сообщение. Обратите внимание, что нам не нужно вызывать метод `drop` явно.

Когда мы запустим эту программу, мы увидим следующий вывод:

```text
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

Rust автоматически вызывает `drop`, когда наши экземпляры выходят из области видимости, вызывая указанный нами код. Переменные удаляются в обратном порядке их создания, поэтому `d` удалено до `c`. Этот пример дает наглядное руководство о том, как работает метод `drop`; обычно вы указываете код очистки, который должен выполнить ваш тип, вместо печати сообщения как в этом примере.

### Раннее удаление значения с помощью `std::mem::drop`

К сожалению, отключение функции автоматического удаления с помощью `drop` является не простым. Отключение `drop` обычно не требуется; весь смысл типажа `Drop` том, чтобы о функции позаботились автоматически. Иногда, однако, вы можете захотеть очистить значение рано. Одним из примеров является использование интеллектуальных указателей, которые управляют блокировками: вы могли бы потребовать принудительный вызов метода `drop` который снимает блокировку, чтобы другой код в той же области видимости мог получить блокировку. Rust не позволяет вызвать метод типажа `Drop` вручную; вместо этого вы должны вызвать функцию `std::mem::drop` предоставляемую стандартной библиотекой, если хотите принудительно удалить значение до конца области видимости.

Мы, конечно же, можем вызывать данный метод явно. Но в этом нет никакого смысла. В Главе 16 мы поговорим о случаях необходимого вызова данного метода заранее (при работе в многопоточной среде). Сейчас же рассмотрим пример явного вызова данного метода:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Листинг 15-15: Попытка вызвать метод <code>drop</code> из типажа <code>Drop</code> вручную для ранней очистки</span>

Когда мы попытаемся скомпилировать этот код, мы получим ошибку:

```text
error[E0040]: explicit use of destructor method
  --> src/main.rs:14:7
   |
14 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
```

This error message states that we’re not allowed to explicitly call `drop`. The
error message uses the term *destructor*, which is the general programming term
for a function that cleans up an instance. A *destructor* is analogous to a
*constructor*, which creates an instance. The `drop` function in Rust is one
particular destructor.

Обратите внимание, что мы вызвали метод `drop`. Если мы вызовем метод `c.drop()` мы получим ошибку компиляции. Нельзя вызывать метод `Drop::drop`, т.к. в этом случае этот метод может быть вызван дважды. Вместо этого мы можем вызвать метод `std::mem::drop`. Определение метода `std::mem::drop`:

Мы не можем отключить автоматическую вставку `drop` кода, когда значение выходит из области видимости и мы не можем явно вызвать метод `drop`. Таким образом, если нам нужно принудительно очистить значение, мы можем использовать функцию `std::mem::drop`.

The `std::mem::drop` function is different from the `drop` method in the `Drop`
trait. We call it by passing the value we want to force to be dropped early as
an argument. The function is in the prelude, so we can modify `main` in Listing
15-15 to call the `drop` function, as shown in Listing 15-16:

<span class="filename">Файл: src/main.rs</span>

```rust
# struct CustomSmartPointer {
#     data: String,
# }
#
# impl Drop for CustomSmartPointer {
#     fn drop(&mut self) {
#         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
#     }
# }
#
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Листинг 15-16: Вызов <code>std::mem::drop</code> для явного удаления значения до его выхода из области видимости</span>

Выполнение данного кода выведет следующий результат::

```text
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

The text `Dropping CustomSmartPointer with data `some data`!` is printed
between the `CustomSmartPointer created.` and `CustomSmartPointer dropped before the end of main.` text, showing that the `drop` method code is called to
drop `c` at that point.

Вы можете использовать код, указанный в реализации типажа `Drop`, чтобы сделать очистку удобной и безопасной: например, вы можете использовать ее для создания своего собственного менеджера памяти! С помощью типажа `Drop` и системы владения Rust не нужно специально заботиться о том, чтобы освобождать ресурсы, потому что Rust делает это автоматически.

You also don’t have to worry about problems resulting from accidentally
cleaning up values still in use: the ownership system that makes sure
references are always valid also ensures that `drop` gets called only once when
the value is no longer being used.

После того, как мы познакомились с `Box<T>` и характеристиками умных указателей, познакомимся с её другими умными указателями, определенными в стандартной библиотеке.
