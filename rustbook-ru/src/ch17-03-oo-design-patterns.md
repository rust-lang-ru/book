## Реализация ООП шаблона проектирования

Шаблон *состояние* является объектно-ориентированным шаблоном проектирования. Суть шаблона в том, что главный объект имеет некоторое внутреннее состояние, которое представлено набором *объектов состояния* и поведение этого объекта изменяется в зависимости от внутреннего состояния. Объекты состояния имеют общую функциональность: конечно в Rust мы используем структуры и типажи, а не объекты и наследование. Каждый объект состояния отвечает за своё поведение и определяет, когда он должен перейти в другое состояние. Значение, которое содержит объект состояния, ничего не знает о различном поведении состояний или о моментах перехода между состояниями.

Использование шаблона состояние означает, что при изменении бизнес-требований программы не нужно изменять код объекта, содержащего состояние или код использующий такой объект. Нам нужно только обновить код внутри одного из значений состояния, чтобы изменить его правила или, возможно, добавить больше значений состояния. Давайте рассмотрим пример проектирования шаблона состояние и как его использовать в Rust.

Мы поэтапно реализуем процесс создания поста в блоге. Окончательная функциональность блога будет выглядеть так:

1. При создании сообщения публикации она создаётся как пустой черновик.
2. Когда черновик готов, запрашивается его рецензия.
3. После проверки происходит её публикация.
4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

Любые другие изменения, сделанные в сообщении, не должны иметь никакого эффекта. Например, если мы попытаемся подтвердить черновик сообщения в блоге до того как запросим рецензию, то сообщение должно остаться неопубликованным черновиком.

В листинге 17-11 показан этот рабочий процесс в виде кода: это пример использования API, который мы реализуем в библиотеку с именем `blog`. Он ещё не компилируется, потому что мы ещё не реализовали крейт для `blog`.

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:all}}
```

<span class="caption">Листинг 17-11: Код, демонстрирующий желаемое поведение, которое мы хотим получить в крейте <code>blog</code></span>

We want to allow the user to create a new draft blog post with `Post::new`. We want to allow text to be added to the blog post. If we try to get the post’s content immediately, before approval, we shouldn't get any text because the post is still a draft. We’ve added `assert_eq!` in the code for demonstration purposes. An excellent unit test for this would be to assert that a draft blog post returns an empty string from the `content` method, but we’re not going to write tests for this example.

Далее мы хотим разрешить сделать запрос на проверку публикации и хотим, чтобы `content` возвращал пустую строку в ожидании проверки. Когда сообщение получит одобрение, оно должно быть опубликовано, то есть текст сообщения будет возвращён при вызове `content`.

Обратите внимание, что единственный тип из крейта, с которым мы взаимодействуем - это тип `Post`. Этот тип будет использовать шаблон состояния и будет содержать значение, которое будет одним из трёх состояний объекта, представляющих различные состояния в которых может находиться публикация: черновик, сообщение ожидающее проверки или опубликованное сообщение. Переход из одного состояния в другое будет осуществляться внутри типа `Post`. Состояния изменяются в ответ на методы, вызываемые пользователями нашей библиотеки у экземпляра `Post`, но они не должны напрямую управлять изменениями состояния. Кроме того, пользователи не могут ошибиться с состояниями, например, опубликовать сообщение до его проверки.

### Определение `Post` и создание нового экземпляра в состоянии черновика

Приступим к реализации библиотеки! Мы знаем, что нам нужна публичная структура `Post`, которая содержит некоторое содержимое, поэтому мы начнём с определения структуры и связанную с ней публичную функцию `new` для создания экземпляра `Post`, как показано в листинге 17-12. Мы также сделаем приватный типаж `State`. Затем `Post` будет содержать типаж-объект `Box<dyn State>` внутри `Option<T>` в приватном поле с названием `state`. Вскоре вы поймёте, почему `Option<T>` необходим. <span>Файл: src/lib.rs</span>

<span class="filename">Листинг 17-12: Определение структуры <code>Post</code> и функции <code>new</code>, которая создаёт новый экземпляр <code>Post</code>, типаж <code>State</code> и структура <code>Draft</code></span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-12/src/lib.rs}}
```

<span class="caption">Листинг 17-12. Определение структуры <code>Post</code> и функции <code>new</code>, которая создаёт новый экземпляр <code>Post</code>, типаж <code>State</code> и структуру <code>Draft</code></span>

Типаж `State` определяет поведение совместно используемое различными состояниями в сообщениях, все типы состояний вроде `Draft`, `PendingReview` и `Published` будут реализовывать типаж `State`. Пока у этого типажа нет никаких методов и мы начнём с определения только `Draft` состояния, потому что это то состояние, в котором мы хотим сделать появление публикации.

Когда мы создаём новый экземпляр `Post`, мы устанавливаем его поле `state` в значение `Some`, содержащее `Box`. Этот `Box` указывает на новый экземпляр структуры `Draft`. Это гарантирует, что всякий раз, когда мы создаём новый экземпляр `Post`, он появляется как черновик. Поскольку поле `state` в структуре `Post` является приватным, то нет никакого способа создать `Post` в любом другом состоянии! В функции `Post::new` мы устанавливаем в поле `content` новую пустую строку `String`.

### Хранение текста содержимого публикации

В листинге 17-11 показано, что мы хотим иметь возможность вызывать метод `add_text` и передать ему `&str`, которое добавляется к текстовому содержимому публикации блога. Мы реализуем эту возможность как метод, а не предоставляем поле `content` как публично доступное `pub`. Это означает, что позже мы сможем реализовать метод, который будет контролировать, как читаются данные из поля `content`. Метод `add_text` довольно прост, поэтому давайте добавим его реализацию в листинге 17-13 в `impl Post`:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-13/src/lib.rs:here}}
```

<span class="caption">Листинг 17-13. Реализация <code>add_text</code> для добавления текста к <code>content</code></span>

Метод `add_text` принимает изменяемую ссылку на `self`, потому что мы меняем экземпляр `Post` для которого вызываем `add_text`. Затем мы вызываем `push_str` для `String` у `content` и передаём `text` аргументом для добавления к сохранённому `content`. Это поведение не зависит от состояния, в котором находится сообщение, поэтому оно не является частью шаблона состояния. Метод `add_text` вообще не взаимодействует с полем `state`, но это часть поведения, которое мы хотим поддерживать.

### Гарантирование пустого содержания черновика сообщения

Даже после того, как мы вызвали `add_text` и добавили некоторый контент в наше сообщение, мы хотим чтобы метод `content` возвращал пустой фрагмент строки, потому что публикация находится ещё в черновом состоянии, как это показано в строке 7 листинга 17-11. А пока давайте реализуем метод `content` с самым простым функционалом, который будет выполнять это требование: всегда возвращать пустой фрагмент строки. Мы изменим код позже, как только реализуем возможность изменить состояние сообщения, чтобы оно могло быть опубликовано. Пока что сообщения могут находиться только в черновом состоянии, поэтому содержимое сообщения всегда должно быть пустым. Листинг 17-14 показывает пустую реализацию:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-14/src/lib.rs:here}}
```

<span class="caption">Листинг 17-14. Добавление реализации заполнителя для метода <code>content</code> в <code>Post</code>, который всегда возвращает пустой фрагмент строки.</span>

С этим дополнением `content` все в листинге 17-11 до строки 7 работает, как задумано.

### Запрос проверки публикации меняет её состояние

Далее нам нужно добавить функциональность запроса рецензии публикации, который должен изменить её состояние с `Draft` на `PendingReview`. Листинг 17-15 показывает этот код:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-15/src/lib.rs:here}}
```

<span class="caption">Листинг 17-15. Реализация методов <code>request_review</code> в <code>Post</code> и <code>State</code></span>

Мы предоставляем в `Post` публичный метод с именем `request_review`, который будет принимать изменяемую ссылку на `self`. Затем мы вызываем внутренний метод `request_review` у поля для текущего состояния `Post` и этот второй метод `request_review` поглощает текущее состояние и возвращает новое состояние.

Мы добавили метод `request_review` `State`; все типы, реализующие этот типаж, теперь должны будут реализовать метод `request_review`. Обратите внимание, что вместо `self`, `&self` или `&mut self` в качестве первого параметра метода у нас `self: Box<Self>`. Этот синтаксис означает, что метод действителен только при вызове `Box` содержащего тип. Этот синтаксис становится владельцем `Box<Self>`, делая старое состояние недействительным, поэтому значение состояния `Post` может преобразоваться в новое состояние.

To consume the old state, the `request_review` method needs to take ownership of the state value. This is where the `Option` in the `state` field of `Post` comes in: we call the `take` method to take the `Some` value out of the `state` field and leave a `None` in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the `state` value out of `Post` rather than borrowing it. Then we’ll set the post’s `state` value to the result of this operation.

We need to set `state` to `None` temporarily rather than setting it directly with code like `self.state = self.state.request_review();` to get ownership of the `state` value. This ensures `Post` can’t use the old `state` value after we’ve transformed it into a new state.

The `request_review` method on `Draft` needs to return a new, boxed instance of a new `PendingReview` struct, which represents the state when a post is waiting for a review. The `PendingReview` struct also implements the `request_review` method but doesn’t do any transformations. Rather, it returns itself, because when we request a review on a post already in the `PendingReview` state, it should stay in the `PendingReview` state.

Now we can start seeing the advantages of the state pattern: the `request_review` method on `Post` is the same no matter its `state` value. Each state is responsible for its own rules.

We’ll leave the `content` method on `Post` as is, returning an empty string slice. We can now have a `Post` in the `PendingReview` state as well as in the `Draft` state, but we want the same behavior in the `PendingReview` state. Listing 17-11 now works up to line 10!

### Добавление метода `approve` который изменяет поведение `content`

Метод `approve` будет аналогичен методу `request_review`: он будет устанавливать у `state` значение, которое должна иметь публикация при её одобрении, как показано в листинге 17-16:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-16/src/lib.rs:here}}
```

<span class="caption">Listing 17-16: Implementing the <code>approve</code> method on <code>Post</code> and the <code>State</code> trait</span>

Мы добавляем метод `approve` в типаж `State`, добавляем новую структуру, которая реализует этот типаж `State` и структуру для состояния `Published`.

Similar to the way `request_review` on `PendingReview` works, if we call the `approve` method on a `Draft`, it will have no effect because `approve` will return `self`. When we call `approve` on `PendingReview`, it returns a new, boxed instance of the `Published` struct. The `Published` struct implements the `State` trait, and for both the `request_review` method and the `approve` method, it returns itself, because the post should stay in the `Published` state in those cases.

Now we need to update the `content` method on `Post`. We want the value returned from `content` to depend on the current state of the `Post`, so we're going to have the `Post` delegate to a `content` method defined on its `state`, as shown in Listing 17-17:

<span class="filename">Файл: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-17/src/lib.rs:here}}
```

<span class="caption">Listing 17-17: Updating the <code>content</code> method on <code>Post</code> to delegate to a <code>content</code> method on <code>State</code></span>

Поскольку цель состоит в том, чтобы сохранить все эти правила внутри структур, реализующих типаж `State`, мы вызываем метод `content` у значения в поле `state` и передаём экземпляр публикации (то есть `self` ) в качестве аргумента. Затем мы возвращаем значение, которое возвращается при использовании метода `content` у поля `state`.

We call the `as_ref` method on the `Option` because we want a reference to the value inside the `Option` rather than ownership of the value. Because `state` is an `Option<Box<dyn State>>`, when we call `as_ref`, an `Option<&Box<dyn State>>` is returned. If we didn’t call `as_ref`, we would get an error because we can’t move `state` out of the borrowed `&self` of the function parameter.

Затем мы вызываем метод `unwrap` , который как мы знаем в данном месте никогда не паникует, потому что мы знаем, что методы `Post` гарантируют что в `state` будет всегда содержаться значение `Some`, после выполнения методов. Это один из случаев, о которых мы говорили в разделе ["Случаи, когда у вас больше информации, чем у компилятора"](ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler)<!--  --> главы 9, когда мы знаем, что значение `None` невозможно, даже если компилятор не может этого понять.

At this point, when we call `content` on the `&Box<dyn State>`, deref coercion will take effect on the `&` and the `Box` so the `content` method will ultimately be called on the type that implements the `State` trait. That means we need to add `content` to the `State` trait definition, and that is where we’ll put the logic for what content to return depending on which state we have, as shown in Listing 17-18:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-18/src/lib.rs:here}}
```

<span class="caption">Listing 17-18: Adding the <code>content</code> method to the <code>State</code> trait</span>

Мы добавляем реализацию по умолчанию метода `content`, который возвращает пустой фрагмент строки. Это означает, что не нужно реализовывать `content` в структурах `Draft` и `PendingReview`.  Структура `Published` будет переопределять метод `content` и вернёт значение из `post.content`.

Note that we need lifetime annotations on this method, as we discussed in Chapter 10. We’re taking a reference to a `post` as an argument and returning a reference to part of that `post`, so the lifetime of the returned reference is related to the lifetime of the `post` argument.

Мы закончили, теперь все из листинга 17-11 работает! Мы внедрили шаблон состояния с помощью правил процесса публикации блога. Логика, связанная с правилами, живёт в объектах состояний, а не разбросана по всей структуре `Post`.

### Компромиссы шаблона состояние

Мы показали, что Rust способен реализовать объектно-ориентированный шаблон состояние для инкапсуляции различных типов поведения, которые должна иметь публикация в каждом состоянии. Методы в `Post` ничего не знают о различных видах поведения. При такой организации кода, мы должны смотреть только в одном месте, чтобы узнать, как может вести себя опубликованная публикация: в реализации типажа `State` у структуры `Published`.

If we were to create an alternative implementation that didn’t use the state pattern, we might instead use `match` expressions in the methods on `Post` or even in the `main` code that checks the state of the post and changes behavior in those places. That would mean we would have to look in several places to understand all the implications of a post being in the published state! This would only increase the more states we added: each of those `match` expressions would need another arm.

With the state pattern, the `Post` methods and the places we use `Post` don’t need `match` expressions, and to add a new state, we would only need to add a new struct and implement the trait methods on that one struct.

The implementation using the state pattern is easy to extend to add more functionality. To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:

- Add a `reject` method that changes the post’s state from `PendingReview` back to `Draft`.
- Потребуйте два вызова метода `approve` прежде чем состояние можно изменить в `Published`.
- Allow users to add text content only when a post is in the `Draft` state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the `Post`.

One downside of the state pattern is that, because the states implement the transitions between states, some of the states are coupled to each other. If we add another state between `PendingReview` and `Published`, such as `Scheduled`, we would have to change the code in `PendingReview` to transition to `Scheduled` instead. It would be less work if `PendingReview` didn’t need to change with the addition of a new state, but that would mean switching to another design pattern.

Another downside is that we’ve duplicated some logic. To eliminate some of the duplication, we might try to make default implementations for the `request_review` and `approve` methods on the `State` trait that return `self`; however, this would violate object safety, because the trait doesn’t know what the concrete `self` will be exactly. We want to be able to use `State` as a trait object, so we need its methods to be object safe.

Другое дублирование включает в себя аналогичные реализации методов `request_review` и `approve` у  `Post`. Оба метода делегируют реализации одного и того же метода значению поля `state` типа `Option` и устанавливают результатом новое значение поля `state`. Если бы у `Post` было много методов, которые следовали этому шаблону, мы могли бы рассмотреть определение макроса для устранения повторения (смотри раздел ["Макросы"](ch19-06-macros.html#macros)<!--  --> в главе 19).

By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could. Let’s look at some changes we can make to the `blog` crate that can make invalid states and transitions into compile time errors.

#### Кодирование состояний и поведения как типы

Мы покажем вам, как переосмыслить шаблон состояния, чтобы получить другой набор компромиссов. Вместо того, чтобы полностью инкапсулировать состояния и переходы так, чтобы внешний код не знал о них, мы будем кодировать состояния с помощью разных типов. Следовательно, система проверки типов Rust предотвратит попытки использовать черновые публикации, там где разрешены только опубликованные публикации, вызывая ошибки компиляции.

Давайте рассмотрим первую часть `main` в листинге 17-11:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:here}}
```

We still enable the creation of new posts in the draft state using `Post::new` and the ability to add text to the post’s content. But instead of having a `content` method on a draft post that returns an empty string, we’ll make it so draft posts don’t have the `content` method at all. That way, if we try to get a draft post’s content, we’ll get a compiler error telling us the method doesn’t exist. As a result, it will be impossible for us to accidentally display draft post content in production, because that code won’t even compile. Listing 17-19 shows the definition of a `Post` struct and a `DraftPost` struct, as well as methods on each:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-19/src/lib.rs}}
```

<span class="caption">Listing 17-19: A <code>Post</code> with a <code>content</code> method and a <code>DraftPost</code> without a <code>content</code> method</span>

Both the `Post` and `DraftPost` structs have a private `content` field that stores the blog post text. The structs no longer have the `state` field because we’re moving the encoding of the state to the types of the structs. The `Post` struct will represent a published post, and it has a `content` method that returns the `content`.

У нас все ещё есть функция `Post::new`, но вместо возврата экземпляра `Post` она возвращает экземпляр `DraftPost`. Поскольку метод `content` является приватным и нет никаких функций, которые возвращают `Post`, то невозможно сразу создать экземпляр `Post`.

Структура `DraftPost` имеет метод `add_text`, поэтому мы можем добавлять текст к `content` как и раньше, но учтите, что в `DraftPost` не определён метод `content`! Так что теперь программа гарантирует, что все публикации начинаются как черновики, а черновики публикаций не имеют своего контента для отображения. Любая попытка обойти эти ограничения приведёт к ошибке компилятора.

#### Реализация переходов как преобразований в другие типы

Так как же получить опубликованный пост? Мы хотим обеспечить соблюдение правила, согласно которому черновик сообщения должен быть рассмотрен и утверждён до того, как он будет опубликован. Публикация находящаяся в состоянии отложенной рецензии, по-прежнему не должна отображать содержимое. Давайте реализуем эти ограничения, добавив ещё одну структуру, `PendingReviewPost`, определяя метод `request_review` у `DraftPost` возвращающий `PendingReviewPost`, определяя метод `approve` у `PendingReviewPost`, возвращающий `Post`, как показано в листинге 17-20:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-20/src/lib.rs:here}}
```

<span class="caption">Listing 17-20: A <code>PendingReviewPost</code> that gets created by calling <code>request_review</code> on <code>DraftPost</code> and an <code>approve</code> method that turns a <code>PendingReviewPost</code> into a published <code>Post</code></span>

Методы `request_review` и `approve` забирают во владение `self`, таким образом поглощая экземпляры `DraftPost` и `PendingReviewPost`, которые потом преобразуются в `PendingReviewPost` и опубликованное `Post` соответственно. Таким образом, у нас не будет никаких длительных экземпляров `DraftPost` после того, как мы вызвали у них `request_review` и так далее. В структуре `PendingReviewPost` не определён метод `content`, поэтому попытка прочитать его содержимое приводит к ошибке компилятора, как в случае с `DraftPost`. Так как единственным способом получить опубликованный экземпляр `Post` у которого действительно есть объявленный метод `content`, является вызов метода `approve` у экземпляра `PendingReviewPost`, то единственный способ получить `PendingReviewPost` это вызвать метод `request_review` у экземпляра `DraftPost`. Так мы реализовали процесс публикации блога с помощью системы типов.

But we also have to make some small changes to `main`. The `request_review` and `approve` methods return new instances rather than modifying the struct they’re called on, so we need to add more `let post =` shadowing assignments to save the returned instances. We also can’t have the assertions about the draft and pending review posts’ contents be empty strings, nor do we need them: we can’t compile code that tries to use the content of posts in those states any longer. The updated code in `main` is shown in Listing 17-21:

<span class="filename">Файл: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-21/src/main.rs}}
```

<span class="caption">Листинг 17-21: Изменения в <code>main</code> для использования новой реализации процесса публикации блога</span>

The changes we needed to make to `main` to reassign `post` mean that this implementation doesn’t quite follow the object-oriented state pattern anymore: the transformations between the states are no longer encapsulated entirely within the `Post` implementation. However, our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time! This ensures that certain bugs, such as display of the content of an unpublished post, will be discovered before they make it to production.

Попробуйте в крейте `blog` сделать предложенные задачи с дополнительными требованиями, которые мы упомянули в начале этого раздела, как это происходит после листинга 17-20, чтобы увидеть и подумать о дизайне версии этого кода. Обратите внимание, что некоторые задачи могут быть выполнены уже в этом проекте.

We’ve seen that even though Rust is capable of implementing object-oriented design patterns, other patterns, such as encoding state into the type system, are also available in Rust. These patterns have different trade-offs. Although you might be very familiar with object-oriented patterns, rethinking the problem to take advantage of Rust’s features can provide benefits, such as preventing some bugs at compile time. Object-oriented patterns won’t always be the best solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.

## Итоги

No matter whether or not you think Rust is an object-oriented language after reading this chapter, you now know that you can use trait objects to get some object-oriented features in Rust. Dynamic dispatch can give your code some flexibility in exchange for a bit of runtime performance. You can use this flexibility to implement object-oriented patterns that can help your code’s maintainability. Rust also has other features, like ownership, that object-oriented languages don’t have. An object-oriented pattern won’t always be the best way to take advantage of Rust’s strengths, but is an available option.

Next, we’ll look at patterns, which are another of Rust’s features that enable lots of flexibility. We’ve looked at them briefly throughout the book but haven’t seen their full capability yet. Let’s go!
