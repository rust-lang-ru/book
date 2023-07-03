## Ссылки и заимствование

Проблема с кодом кортежа в листинге 4-5 заключается в том, что мы должны вернуть `String` из вызванной функции, чтобы использовать `String` после вызова `calculate_length`, потому что `String` была перемещена в `calculate_length`. Вместо этого мы можем предоставить ссылку на значение `String`. *Ссылка* похожа на указатель в том смысле, что это адрес, по которому мы можем проследовать, чтобы получить доступ к данным, хранящимся по этому адресу; эти данные принадлежат какой-то другой переменной. В отличие от указателя, ссылка гарантированно указывает на допустимое значение определённого типа в течение всего срока существования этой ссылки.

Вот как вы могли бы определить и использовать функцию `calculate_length`, имеющую ссылку на объект в качестве параметра, вместо того, чтобы брать на себя ответственность за значение:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

Во-первых, обратите внимание, что весь код кортежа в объявлении переменной и возвращаемое значение функции исчезли. Во-вторых, обратите внимание, что мы передаём `&s1` в `calculate_length` и в его определении используем `&String`, а не `String`. Эти амперсанды представляют собой *ссылки*, и они позволяют вам ссылаться на некоторое значение, не принимая владение над ним. Рисунок 4-5 изображает эту концепцию.

<img alt="&amp;String s pointing at String s1" src="img/trpl04-05.svg" class="">

<span class="caption">Рисунок 4-5: диаграмма для <code>&amp;String s</code>, указывающей на <code>String s1</code></span>

> Примечание: противоположностью ссылки с использованием `&` является *разыменование*, выполняемое с помощью оператора разыменования `*`. Мы увидим некоторые варианты использования оператора разыменования в главе 8 и обсудим детали разыменования в главе 15.

Давайте подробнее рассмотрим механизм вызова функции:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&s1` позволяет нам создать ссылку, которая *ссылается* на значение `s1`, но не владеет им. Поскольку она не владеет им, значение, на которое она указывает, не будет удалено, когда ссылка перестанет использоваться.

Сигнатура функции использует `&` для индикации того, что тип параметра `s` является ссылкой. Добавим объясняющие комментарии:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

The scope in which the variable `s` is valid is the same as any function parameter’s scope, but the value pointed to by the reference is not dropped when `s` stops being used, because `s` doesn’t have ownership. When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

Мы называем процесс создания ссылки *заимствованием*. Как и в реальной жизни, если человек чем-то владеет, вы можете это у него позаимствовать. Когда вы закончите, вы должны вернуть это законному владельцу.

Так что же произойдёт, если мы попытаемся изменить что-то, что мы заимствуем? Попробуйте запустить код из листинга 4-6. Спойлер: это не сработает!

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">Листинг 4-6: попытка модификации заимствованной переменной</span>

Вот ошибка:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

### Изменяемые ссылки

Мы можем исправить код из листинга 4-6, чтобы позволить себе изменять заимствованное значение, с помощью нескольких небольших настроек, которые используют *изменяемую ссылку*:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

First we change `s` to be `mut`. Then we create a mutable reference with `&mut s` where we call the `change` function, and update the function signature to accept a mutable reference with `some_string: &mut String`. This makes it very clear that the `change` function will mutate the value it borrows.

Изменяемые ссылки имеют одно большое ограничение: если у вас есть изменяемая ссылка на значение, у вас не может быть других ссылок на это же значение. Код, который пытается создать две изменяемые ссылки на `s`, завершится ошибкой:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Описание ошибки:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Эта ошибка говорит о том, что код недействителен, потому что мы не можем заимствовать `s` как изменяемые более одного раза в один момент. Первое изменяемое заимствование находится в `r1` и должно длиться до тех пор, пока оно не будет использовано в `println!`, но между созданием этой изменяемой ссылки и её использованием мы попытались создать другую изменяемую ссылку в `r2`, которая заимствует те же данные, что и `r1`.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A *data race* is similar to a race condition and happens when these three behaviors occur:

- Два или больше указателей используют одни и те же данные в одно и то же время,
- Минимум один указатель используется для записи данных,
- Отсутствуют механизмы для синхронизации доступа к данным.

Гонки данных вызывают неопределённое поведение, и их может быть сложно диагностировать и исправить, когда вы пытаетесь отследить их во время выполнения. Rust предотвращает такую проблему, отказываясь компилировать код с гонками данных!

Как всегда, мы можем использовать фигурные скобки для создания новой области видимости, позволяющей использовать несколько изменяемых ссылок, но не *одновременно*:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust применяет аналогичное правило для комбинирования изменяемых и неизменяемых ссылок. Этот код приводит к ошибке:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Ошибка:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Вау! У нас *также* не может быть изменяемой ссылки, пока у нас есть неизменяемая ссылка на то же значение.

Пользователи неизменяемой ссылки не ожидают, что значение внезапно изменится из-под них! Однако разрешены множественные неизменяемые ссылки, потому что никто, кто просто читает данные, не может повлиять на чтение данных кем-либо ещё.

Обратите внимание, что область действия ссылки начинается с того места, где она была введена, и продолжается до последнего использования этой ссылки. Например, этот код будет компилироваться, потому что последнее использование неизменяемых ссылок `println!`, происходит до того, как вводится изменяемая ссылка:

```rust,edition2021
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Области неизменяемых ссылок `r1` и `r2` заканчиваются после `println!` где они использовались в последний раз, то есть до создания изменяемой ссылки `r3`. Эти области не перекрываются, поэтому этот код разрешён: компилятор может сказать, что ссылка больше не используется в точке перед концом области.

Even though borrowing errors may be frustrating at times, remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is. Then you don’t have to track down why your data isn’t what you thought it was.

### Висячие ссылки

В языках с указателями весьма легко ошибочно создать недействительную, висячую *(dangling)* ссылку. Ссылку указывающую на участок памяти, который мог быть передан кому-то другому, путём освобождения некоторой памяти при сохранении указателя на эту память. Rust компилятор гарантирует, что ссылки никогда не станут недействительными: если у вас есть ссылка на какие-то данные, компилятор обеспечит что эти данные не выйдут из области видимости прежде, чем из области видимости исчезнет ссылка.

Давайте попробуем создать висячую ссылку, чтобы увидеть, как Rust предотвращает их появление с помощью ошибки во время компиляции:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Здесь ошибка:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

This error message refers to a feature we haven’t covered yet: lifetimes. We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Let’s take a closer look at exactly what’s happening at each stage of our `dangle` code:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

Поскольку `s` создаётся внутри `dangle`, когда код `dangle` будет завершён, `s` будет освобождена. Но мы попытались вернуть ссылку на неё. Это означает, что эта ссылка будет указывать на недопустимую `String`. Это нехорошо! Rust не позволит нам сделать это.

Решением будет вернуть непосредственно `String`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

### Правила работы с ссылками

Давайте повторим  все, что мы обсудили про ссылки:

- В любой момент времени у вас может быть *одна* (но не обе) одна изменяемая ссылка или любое количество неизменяемых ссылок.
- Все ссылки должны быть действительными.

В следующей главе мы рассмотрим другой тип ссылок — срезы.
