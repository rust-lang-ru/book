## Запуск кода при очистке с помощью типажа `Drop`

Второй типаж, важный для шаблона умного указателя, - это `Drop`, который позволяет настроить то, что происходит, когда значение собирается выйти из области видимости. Вы можете предоставить реализацию `Drop` для любого типа, а указанный код можно использовать для освобождения ресурсов, таких как файлы или сетевые подключения. Мы представим `Drop` в контексте умных указателей, потому что функциональность типажа `Drop` почти всегда используется при их реализации. Например, `Box<T>` настраивает <code>Drop</code> для освобождения пространства в куче, на которое указывает Box.

В некоторых языках программист должен вызывать код для освобождения памяти или ресурсов каждый раз, когда они заканчивают использовать экземпляр умного указателя. Если программист забудет это сделать, то система может стать перегруженной и начать зависать. В Rust можно указать, что определённый код будет запускаться всякий раз, когда значение выходит из области видимости и компилятор вставит такой код автоматически. В результате вам не нужно заботиться о размещении кода очистки ресурсов во всей программе, где экземпляр определённого типа заканчивает существование - вы не потеряете системные ресурсы!

Укажите код, который будет запускаться, когда значение выходит из области видимости с помощью реализации типажа `Drop`. Типаж `Drop` требует, чтобы вы реализовали один метод с именем `drop`, который принимает изменяемую ссылку на `self`. Чтобы увидеть, когда Rust вызывает метод `drop`, давайте реализуем `drop` с помощью оператора `println!`.

В листинге 15-14 показана структура `CustomSmartPointer`, единственная функция которой заключается в том, что она будет печатать `Dropping CustomSmartPointer!` когда экземпляр выходит за область видимости. Этот пример демонстрирует, когда Rust выполняет функцию `drop` .

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<span class="caption">Листинг 15-14: Структура <code>CustomSmartPointer</code>, которая реализует типаж <code>Drop</code>, где мы поместили бы код очистки</span>

Типаж `Drop` входит в прелюдию, поэтому не нужно подключать его в область видимости. Мы реализуем типаж `Drop` у структуры `CustomSmartPointer` и предоставляем реализацию для метода `drop`, который вызывает `println!`. В теле функции `drop` можно разместить любую логику, которую хочется запустить, когда экземпляр вашего типа выходит из области видимости. Здесь мы печатаем некоторый текст, чтобы продемонстрировать, когда Rust вызовет `drop`.

В `main` мы создаём два экземпляра `CustomSmartPointer` и затем печатаем `CustomSmartPointers created` . В конце `main` наши экземпляры `CustomSmartPointer` выйдут из области видимости и Rust вызовет код, который мы добавили в метод `drop`, который и напечатает наше окончательное сообщение. Обратите внимание, что нам не нужно вызывать метод `drop` явно.

Когда мы запустим эту программу, мы увидим следующий вывод:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust автоматически вызывает `drop`, когда наши экземпляры выходят из области видимости, вызывая указанный нами код. Переменные удаляются в обратном порядке их создания, поэтому `d` удалено до `c`. Этот пример даёт наглядное руководство о том, как работает метод `drop`; обычно вы указываете код очистки, который должен выполнить ваш тип, вместо печати сообщения как в этом примере.

### Раннее удаление значения с помощью `std::mem::drop`

К сожалению, отключение функции автоматического удаления с помощью `drop` является не простым. Отключение `drop` обычно не требуется; весь смысл типажа `Drop` том, чтобы о функции позаботились автоматически. Иногда, однако, вы можете захотеть очистить значение рано. Одним из примеров является использование интеллектуальных указателей, которые управляют блокировками: вы могли бы потребовать принудительный вызов метода `drop` который снимает блокировку, чтобы другой код в той же области видимости мог получить блокировку. Rust не позволяет вызвать метод типажа `Drop` вручную; вместо этого вы должны вызвать функцию `std::mem::drop` предоставляемую стандартной библиотекой, если хотите принудительно удалить значение до конца области видимости.

Если попытаться вызвать метод `drop` типажа `Drop` вручную, изменяя функцию `main` листинга 15-14 так, как показано в листинге 15-15, мы получим ошибку компилятора:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<span class="caption">Листинг 15-15: Попытка вызвать метод <code>drop</code> из типажа <code>Drop</code> вручную для ранней очистки</span>

Когда мы попытаемся скомпилировать этот код, мы получим ошибку:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Это сообщение об ошибке говорит, что мы не можем явно вызывать `drop`. В сообщении об ошибке используется термин *деструктор (destructor)*, который является общим термином программирования для функции, которая очищает экземпляр. *Деструктор* аналогичен *конструктору*, который создаёт экземпляр. Функция `drop` в Rust является определённым деструктором.

Rust не позволяет явно вызывать `drop`, потому что Rust всё равно будет автоматически вызывать `drop` для значения в конце `main`. Это приведёт к ошибке *двойного освобождения*, потому что Rust будет пытаться очистить одно и то же значение дважды.

Мы не можем отключить автоматическую вставку `drop` кода, когда значение выходит из области видимости и мы не можем явно вызвать метод `drop`. Таким образом, если нам нужно принудительно очистить значение, мы можем использовать функцию `std::mem::drop`.

Функция `std::mem::drop` отличается от метода `drop` типажа `Drop`. Мы вызываем её, передавая значение, которое мы хотим принудительно освободить в качестве аргумента. Функция находится в прелюдии, поэтому можно изменить `main` из листинга 15-15 так, чтобы вызвать функцию `drop`, как показано в листинге 15-16:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<span class="caption">Листинг 15-16: Вызов <code>std::mem::drop</code> для явного удаления значения до его выхода из области видимости</span>

Выполнение данного кода выведет следующий результат::

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

Текст `Dropping CustomSmartPointer with data `some data`!`, напечатанный между `CustomSmartPointer created.` и текстом `CustomSmartPointer dropped before the end of main.`, показывает, что код метода `drop` вызывается для удаления <code>c</code> в этой точке.

Вы можете использовать код, указанный в реализации типажа `Drop`, чтобы сделать очистку удобной и безопасной: например, вы можете использовать её для создания своего собственного менеджера памяти! С помощью типажа `Drop` и системы владения Rust не нужно специально заботиться о том, чтобы освобождать ресурсы, потому что Rust делает это автоматически.

Также не нужно беспокоиться о проблемах, возникающих в результате случайной очистки значений, которые всё ещё используются: система владения, которая гарантирует, что ссылки всегда действительны, также гарантирует, что `drop` вызывается только один раз, когда значение больше не используется.

После того, как мы познакомились с `Box<T>` и характеристиками умных указателей, познакомимся с её другими умными указателями, определёнными в стандартной библиотеке.
