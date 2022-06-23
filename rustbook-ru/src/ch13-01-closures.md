<!-- Old heading. Do not remove or links may break. -->
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Замыкания: анонимные функции, которые могут захватывать окружение

Замыкания Rust - это анонимные функции, которые вы можете сохранить в переменной или передать в качестве аргументов другим функциям. Вы можете создать замыкание в одном месте, а затем вызвать замыкание, чтобы вычислить его в другом контексте. В отличие от функций, замыкания могут захватывать значения из области видимости где они определены. Мы продемонстрируем, как эти возможности замыканий позволяют повторно использовать код и настраивать поведение.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Захват окружения используя замыкания

Сначала мы рассмотрим, как мы можем использовать замыкания для захвата значений из среды, в которой они определены, для последующего использования. Вот сценарий: время от времени наша компания по производству футболок дарит эксклюзивную футболку ограниченного выпуска кому-то из нашего списка рассылки в качестве рекламной акции. Люди из списка рассылки могут при желании добавить свой любимый цвет в свой профиль. Если человек, выбранный для бесплатной рубашки, имеет свой любимый набор цветов, он получает рубашку этого цвета. Если человек не указал любимый цвет, он получает тот цвет, которого в настоящее время больше всего в компании.

Есть много способов реализовать это. В этом примере мы собираемся использовать перечисление под названием `ShirtColor`, которое имеет варианты `Red` и `Blue` (ограничим количество доступных цветов для простоты). Мы представляем каталог компании с помощью структуры `Inventory`, в которой есть поле с именем `shirts`, которое содержит`Vec<ShirtColor>`, представляющий цвета рубашек, которые в настоящее время есть на складе. Метод `shirt_giveaway`, определённый в `Inventory`, получает необязательное предпочтение цвета рубашки победителя бесплатной футболки и возвращает цвет рубашки, который получит человек. Этот алгоритм показан в листинге 13-1:

<span class="filename">Файл: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Листинг 13-1: Акция с раздачей фирменных футболок</span>

В `store`, определённым в `main`, остались две синие и одна красная футболки, которые можно подарить в рамках этой акции с ограниченным тиражом. Мы вызываем метод `giveaway` для пользователя, предпочитающего красную футболку, и пользователя без каких-либо предпочтений.

Опять же, этот код может быть реализован разными способами, и здесь, чтобы сосредоточиться на замыканиях, мы придерживались концепций, которые вы уже изучили, за исключением тела метода `giveaway`, который использует замыкание. В методе `giveaway` мы получаем предпочтения пользователя как параметр типа `Option<ShirtColor>` и вызываем метод `unwrap_or_else` для `user_preference`. [Метод `unwrap_or_else` для `Option<T>`][unwrap-or-else]<!-- ignore --> определяется стандартной библиотекой. Он принимает один аргумент: замыкание без каких-либо аргументов, которое возвращает значение `T` (тот же тип, который хранится в `Some` варианте `Option<T>`, в данном случае `ShirtColor`). Если `Option<T>` является вариантом `Some`, `unwrap_or_else` возвращает значение из `Some`. Если `Option<T>` является вариантом `None`, `unwrap_or_else` вызывает замыкание и возвращает значение, возвращённое замыканием.

Мы указываем замыкающее выражение `|| self.most_stocked()` в качестве аргумента для `unwrap_or_else`. Это замыкание, которое само не принимает параметров (если бы замыкание имело параметры, они отображались бы между двумя вертикальными полосами). Тело замыкания вызывает `self.most_stocked()`. Здесь мы определяем замыкание, и реализация `unwrap_or_else` вызовет замыкание позже, если потребуется результат.

При выполнении этот код выводит:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Один интересный аспект здесь заключается в том, что мы передали замыкание, которое вызывает
`self.most_stocked()` для текущего экземпляра `Inventory`. Стандартной библиотеке не нужно ничего знать об определённых нами типах `Inventory` или `ShirtColor`, или о логике, которую мы хотим использовать в этом сценарии. Замыкание захватывает неизменяемую ссылку `self` на экземпляр `Inventory` и передаёт её в код, который мы указываем в методе `unwrap_or_else`. С другой стороны, (обычные) функции не могут таким образом захватывать своё окружение.

### Выведение типа замыкания и аннотация

Есть различия между функциями и замыканиями. Замыкания обычно не требуют, чтобы вы аннотировали типы параметров или возвращаемое значение, как это делается для функций `fn`. Аннотации типов необходимы для функций, поскольку типы являются частью явного интерфейса, доступного вашим пользователям. Жёсткое определение этого интерфейса важно для гарантии того, чтобы каждый понимал какие типы значений использует и возвращает функция. Замыкания, с другой стороны, не используются в открытом интерфейсе, подобном этому: они хранятся в переменных и используются без их именования и предоставления пользователям нашей библиотеки.

Замыкания обычно короткие и актуальны только в узком контексте, а не в произвольном сценарии. В этих ограниченных контекстах компилятор может вывести типы параметров и тип возвращаемого значения, аналогично тому, как он может вывести типы большинства переменных (в редких случаях компилятору также нужны аннотации типа замыкания).

Как и в случае с переменными, мы можем добавить аннотации типов, если хотим повысить чёткость и ясность за счёт большей детализации, чем это необходимо. Аннотирование типов для замыкания будет выглядеть так, как показано в листинге 13-2. В этом примере мы определяем замыкание и сохраняем его в переменной, а не определяем замыкание в месте, где мы передаём его в качестве аргумента, как в листинге 13-1.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">Листинг 13-2: Добавление необязательных аннотаций типов для параметров и типов возвращаемых значений у замыкания</span>

С добавленными аннотациями типов синтаксис замыканий выглядит более похожим на синтаксис функций. Ниже приводится сравнение синтаксиса функции, которая добавляет 1 к своему параметру и несколько эквивалентных замыканий, которые имеют аналогичное поведение. Мы добавили несколько пробелов для выравнивания соответствующих частей. Это показывает, как синтаксис замыкания похож на синтаксис функции, за исключением использования вертикальных палочек и необязательного синтаксиса:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Первая строка показывает определение функции, а вторая строка показывает полностью аннотированное определение замыкания. В третьей строке удаляются аннотации типов из определения замыкания, а в четвёртой строке удаляются скобки, которые являются необязательными, поскольку тело замыкания имеет только одно выражение. Все это допустимые определения, которые будут вызывать одинаковое поведение при вызове. Вызов замыканий `add_one_v3` и `add_one_v4` требует, чтобы они могли скомпилироваться, поэтому при компиляции типы будут выводиться на основе их использования. Это похоже на `let v = Vec::new();`, требующий либо аннотаций типа, либо значений некоторого типа, которые должны быть вставлены в `Vec`, чтобы Rust мог вывести тип.

Для определений замыканий компилятор выведет один конкретный тип для каждого из их параметров и для их возвращаемого значения. Например, в листинге 13-3 показано определение короткого замыкания, которое просто возвращает значение, полученное в качестве параметра. Это замыкание не очень полезно, за исключением целей этого примера. Обратите внимание, что мы не добавили в определение никаких аннотаций типов. Поскольку аннотаций типов нет, мы можем вызвать замыкание с любым типом, что мы и сделали здесь со `String` в первый раз. Если мы затем попытаемся вызвать `example_closure` с целым числом, то мы получим ошибку.

<span class="filename">Файл: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Листинг 13-3: Попытка вызвать замыкание, типы у которого выводятся двумя разными типами</span>

Компилятор вернёт нам вот такую ошибку:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

Когда мы в первый раз вызываем `example_closure` со значением типа `String`, компилятор выводит тип для `x` и возвращаемого значения как `String`. Эти типы затем привязываются к замыканию в `example_closure` и мы получаем ошибку типа, если пытаемся использовать другой тип с тем же замыканием.

### Заимствование ссылок и получение права собственности

Замыкания могут захватывать значения из своего окружения тремя способами, которые непосредственно соответствуют трём способам, которыми функция может принимать параметр: неизменное заимствование, изменяемое заимствование и получение права собственности. Замыкание решит, какой из них использовать, основываясь на том, что тело функции делает с захваченными значениями.

В листинге 13-4 мы определяем замыкание, которое захватывает неизменяемую ссылку на вектор с именем `list` потому, что для вывода значения требуется только неизменная ссылка:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">Листинг 13-4: Определение и вызов замыкания, которое захватывает неизменяемую ссылку</span>

Этот пример также показывает, что переменная может быть связана с определением замыкания, и мы можем позже вызвать замыкание, используя имя переменной и круглые скобки, как если бы имя переменной было именем функции.

Поскольку у нас может быть несколько неизменяемых ссылок на `list` одновременно, `list` по-прежнему доступен из кода до определения замыкания, после определения замыкания, но до вызова замыкания и после вызова замыкания. Этот код компилируется, запускается и печатает:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Затем в листинге 13-5 мы меняем тело замыкания, чтобы оно добавляло элемент в вектор `list`. Теперь замыкание захватывает изменяемую ссылку:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">Листинг 13-5: Определение и вызов замыкания, которое захватывает изменяемую ссылку</span>

Этот код компилируется, запускается и печатает:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Обратите внимание, что между определением и вызовом замыкания `borrows_mutably` больше нет `println!`: когда `borrows_mutably` определён, он захватывает изменяемую ссылку на `list`. Мы больше не используем замыкание ещё раз после его вызова, поэтому изменяемое заимствование заканчивается. Между определением и вызовом замыкания недопустимо неизменяемое заимствование для печати, потому что при наличии изменяемого заимствования никакие другие заимствования не допускаются. Попробуйте добавить `println!`, чтобы увидеть, какое сообщение об ошибке вы получите!

Если вы хотите, чтобы замыкание стало владельцем значений, которые оно использует из окружения, даже если тело замыкания строго не требует владения, вы можете использовать ключевое слово `move` перед списком параметров. Этот приём в основном полезен при передаче замыкания в новый поток для перемещения данных, чтобы они принадлежали новому потоку. У нас будет больше примеров замыканий с `move` в Главе 16, когда мы будем говорить о параллелизме.

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Возврат захваченных значений из замыканий и типажи `Fn`

Как только замыкание захватило ссылку или захватило право собственности на значение, где определено замыкание (таким образом влияя на то, не важно что, что перемещается *в* замыкание), код в теле замыкания определяет, что происходит со ссылками или значения, когда замыкание отработает (таким образом влияя на то, не важно что, что перемещается *из* замыкания). Тело замыкания может выполнять одно из следующих действий: перемещать захваченное значение из замыкания, изменять захваченное значение, не перемещать и не изменять значение, или ничего не захватывать из окружения при работе.

То, как замыкание захватывает и обрабатывает значения из окружения, влияет на то, какие трейты реализует замыкание, а трейты — на то, как функции и структуры могут указывать, какие типы замыканий они могут использовать. Замыкания автоматически реализуют один, два или все три `Fn` типажа аддитивным образом:

1. `FnOnce` применяется к замыканиям, которые можно вызвать хотя бы один раз. Все замыкания реализуют по крайней мере этот типаж, потому что все замыкания могут быть вызваны. Замыкание, которое перемещает захваченные значения из своего тела, будет реализовывать только `FnOnce` и ни один из других `Fn` типажей, потому что его можно вызвать только один раз.
2. `FnMut` применяется к замыканиям, которые не перемещают захваченные значения из своего тела, но могут изменять захваченные значения. Эти замыкания можно вызывать более одного раза.
3. `Fn` применяется к замыканиям, которые не перемещают захваченные значения из своего тела и не изменяют захваченные значения, а также к замыканиям, которые ничего не захватывают из своего окружения. Эти замыкания можно вызывать более одного раза без изменения их окружения, что важно в таких случаях, как многократный одновременный вызов замыкания.

Давайте посмотрим на определение метода `unwrap_or_else` для `Option<T>`, которое мы использовали в листинге 13-6:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Напомним, что `T` — это обобщённый тип, представляющий тип значения в варианте `Some` параметра `Option`. Этот тип `T` также является возвращаемым типом функции `unwrap_or_else`: например, код, который вызывает `unwrap_or_else` для `Option<String>`, получит `String`.

Next, notice that the `unwrap_or_else` function has the additional generic type
parameter `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.

The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called at least once, take no arguments, and
return a `T`. Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` is only going to call `f` at most one time. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts the most different kinds
of closures and is as flexible as it can be.

> Note: Functions can implement all three of the `Fn` traits too. If what we
> want to do doesn’t require capturing a value from the environment, we can use
> the name of a function rather than a closure where we need something that
> implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value,
> we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the
> value is `None`.

Now let’s look at the standard library method `sort_by_key` defined on slices,
to see how that differs from `unwrap_or_else` and why `sort_by_key` uses
`FnMut` instead of `FnOnce` for the trait bound.

The closure gets one argument, a reference to the current item in the slice
being considered, and returns a value of type `K` that can be ordered. This
function is useful when you want to sort a slice by a particular attribute of
each item. In Listing 13-x, we have a list of `Rectangle` instances and we use
`sort_by_key` to order them by their `width` attribute from low to high:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">Листинг 13-7: Использование `sort_by_key` для упорядочивания прямоугольников по ширине</span>

Этот код выводит:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn’t capture, mutate, or move out anything from its environment, so
it meets the trait bound requirements.

In contrast, Listing 13-8 shows an example of a closure that implements just
the `FnOnce` trait, because it moves a value out of the environment. The
compiler won’t let us use this closure with `sort_by_key`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">Listing 13-8: Attempting to use an `FnOnce` closure with
`sort_by_key`</span>

This is a contrived, convoluted way (that doesn’t work) to try and count the
number of times `sort_by_key` gets called when sorting `list`. This code
attempts to do this counting by pushing `value`—a `String` from the closure’s
environment—into the `sort_operations` vector. The closure captures `value`
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn’t work because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can’t be moved out of the closure because the closure must
implement `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn’t
move values out of the environment. To count the number of times `sort_by_key`
is called, keeping a counter in the environment and incrementing its value in
the closure body is a more straightforward way to calculate that. The closure
in Listing 13-x works with `sort_by_key` because it is only capturing a mutable
reference to the `num_sort_operations` counter and can therefore be called more
than once:

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">Листинг 13-9: Разрешается использование `FnMut` замыкания с `sort_by_key`</span>

Типажи `Fn` важны при определении и использовании функций или типов, использующих замыкания. В следующем разделе мы обсудим итераторы. Многие методы итератора принимают замыкания как параметры, поэтому помните об этих тонкостях замыканий, пока мы двигаемся дальше!

[unwrap-or-else]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
