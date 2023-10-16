<!-- Old heading. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>

## Управляющая конструкция `match`

Rust has an extremely powerful control flow construct called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things; [Chapter 18](ch18-00-patterns.html)<!-- ignore --> covers all the different kinds of patterns and what they do. The power of `match` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Думайте о выражении `match` как о машине для сортировки монет: монеты скользят по дорожке с различными по размеру отверстиями, и каждая монета падает через первое попавшееся отверстие, в которое она поместилась. Таким же образом значения проходят через каждый шаблон в `match`, и при первом же "подходящем" шаблоне значение попадает в соответствующий блок кода, который будет использоваться во время выполнения.

Speaking of coins, let’s use them as an example using `match`! We can write a function that takes an unknown US coin and, in a similar way as the counting machine, determines which coin it is and returns its value in cents, as shown in Listing 6-3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Листинг 6-3: Перечисление и выражение <code>match</code>, использующее в качестве шаблонов его варианты</span>

Let’s break down the `match` in the `value_in_cents` function. First we list the `match` keyword followed by an expression, which in this case is the value `coin`. This seems very similar to a conditional expression used with `if`, but there’s a big difference: with `if`, the condition needs to evaluate to a Boolean value, but here it can be any type. The type of `coin` in this example is the `Coin` enum that we defined on the first line.

Далее идут ветки `match`. Ветки состоят из двух частей: шаблон и некоторый код. Здесь первая ветка имеет шаблон, который является значением `Coin::Penny`, затем идёт оператор `=>`, который разделяет шаблон и код для выполнения. Код в этом случае - это просто значение `1`. Каждая ветка отделяется от последующей при помощи запятой.

When the `match` expression executes, it compares the resultant value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm, much as in a coin-sorting machine. We can have as many arms as we need: in Listing 6-3, our `match` has four arms.

The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire `match` expression.

Обычно фигурные скобки не используются, если код совпадающей ветви невелик, как в листинге 6-3, где каждая ветвь просто возвращает значение. Если вы хотите выполнить несколько строк кода в одной ветви, вы должны использовать фигурные скобки, а запятая после этой ветви необязательна. Например, следующий код печатает "Lucky penny!" каждый раз, когда метод вызывается с `Coin::Penny`, но при этом он возвращает последнее значение блока - `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Patterns That Bind to Values

Есть ещё одно полезное качество у веток в выражении <code>match</code>: они могут привязываться к частям тех значений, которые совпали с шаблоном. Благодаря этому можно извлекать значения из вариантов перечисления.

As an example, let’s change one of our enum variants to hold data inside it. From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have this extra value. We can add this information to our `enum` by changing the `Quarter` variant to include a `UsState` value stored inside it, which we’ve done in Listing 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Листинг 6-4: Перечисление <code>Coin</code>, в котором вариант <code>Quarter</code> также сохраняет значение <code>UsState</code></span>

Let’s imagine that a friend is trying to collect all 50 state quarters. While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so that if it’s one our friend doesn’t have, they can add it to their collection.

В выражении match для этого кода мы добавляем переменную с именем `state` в шаблон, который соответствует значениям варианта `Coin::Quarter`. Когда `Coin::Quarter` совпадёт с шаблоном, переменная `state` будет привязана к значению штата этого четвертака. Затем мы сможем использовать `state` в коде этой ветки, вот так:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Если мы сделаем вызов функции `value_in_cents(Coin::Quarter(UsState::Alaska))`, то `coin` будет иметь значение `Coin::Quarter(UsState::Alaska)`. Когда мы будем сравнивать это значение с каждой из веток, ни одна из них не будет совпадать, пока мы не достигнем варианта `Coin::Quarter(state)`. В этот момент `state` привяжется к значению `UsState::Alaska`. Затем мы сможем использовать эту привязку в выражении `println!`, получив таким образом внутреннее значение варианта `Quarter` перечисления `Coin`.

### Сопоставление шаблона для `Option<T>`

In the previous section, we wanted to get the inner `T` value out of the `Some` case when using `Option<T>`; we can also handle `Option<T>` using `match`, as we did with the `Coin` enum! Instead of comparing coins, we’ll compare the variants of `Option<T>`, but the way the `match` expression works remains the same.

Допустим, мы хотим написать функцию, которая принимает `Option<i32>` и если есть значение внутри, то добавляет 1 к существующему значению. Если значения нет, то функция должна возвращать значение `None` и не пытаться выполнить какие-либо операции.

Такую функцию довольно легко написать благодаря выражению `match`, код будет выглядеть как в листинге 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Листинг 6-5: Функция, использующая выражение <code>match</code> для <code>Option&lt;i32&gt;</code></span>

Let’s examine the first execution of `plus_one` in more detail. When we call `plus_one(five)`, the variable `x` in the body of `plus_one` will have the value `Some(5)`. We then compare that against each match arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

The `Some(5)` value doesn’t match the pattern `None`, so we continue to the next arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

Does `Some(5)` match `Some(i)`? It does! We have the same variant. The `i` binds to the value contained in `Some`, so `i` takes the value `5`. The code in the match arm is then executed, so we add 1 to the value of `i` and create a new `Some` value with our total `6` inside.

Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is `None`. We enter the `match` and compare to the first arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Оно совпадает! Для данной ветки шаблон (<em>None</em>) не подразумевает наличие какого-то значения к которому можно было бы что-то добавить, поэтому программа останавливается и возвращает значение которое находится справа от `=>` - т.е. `None`. Так как шаблон первой ветки совпал, то никакие другие шаблоны веток не сравниваются.

Комбинирование `match` и перечислений полезно во многих ситуациях. Вы часто будете видеть подобную комбинацию в коде на Rust: сделать сопоставление значений перечисления используя `match`, привязать переменную к данным внутри значения, выполнить код на основе привязанных данных. Сначала это может показаться немного сложным, но как только вы привыкнете, то захотите чтобы такая возможность была бы во всех языках. Это неизменно любимый пользователями приём.

### Match объемлет все варианты значения

Есть ещё один аспект `match`, который мы должны обсудить: шаблоны должны покрывать все возможные варианты. Рассмотрим эту версию нашей функции `plus_one`, которая содержит ошибку и не компилируется:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Мы не обработали вариант `None`, поэтому этот код вызовет дефект в программе. К счастью, Rust знает и умеет ловить такой случай. Если мы попытаемся скомпилировать такой код, мы получим ошибку компиляции:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust knows that we didn’t cover every possible case, and even knows which pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier impossible.

### Универсальные шаблоны и заполнитель `_`

Используя перечисления, мы также можем выполнять специальные действия для нескольких определённых значений, а для всех остальных значений выполнять одно действие по умолчанию. Представьте, что мы реализуем игру, в которой при выпадении 3 игрок не двигается, а получает новую модную шляпу. Если выпадает 7, игрок теряет шляпу. При всех остальных значениях ваш игрок перемещается на столько-то мест на игровом поле. Вот `match`, реализующий эту логику, в котором результат броска костей жёстко закодирован, а не является случайным значением, а вся остальная логика представлена функциями без тел, поскольку их реализация не входит в рамки данного примера:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Для первых двух веток шаблонами являются литеральные значения 3 и 7. Для последней ветки, которая охватывает все остальные возможные значения, шаблоном является переменная, которую мы решили назвать `other`. Код, выполняемый для ветки `other`, использует эту переменную, передавая её в функцию `move_player`.

Этот код компилируется, даже если мы не перечислили все возможные значения `u8`, потому что последний паттерн будет соответствовать всем значениям, не указанным в конкретном списке. Этот универсальный шаблон удовлетворяет требованию, что соответствие должно быть исчерпывающим. Обратите внимание, что мы должны поместить ветку с универсальным шаблоном последней, потому что шаблоны оцениваются по порядку. Rust предупредит нас, если мы добавим ветки после универсального шаблона, потому что эти последующие ветки никогда не будут выполняться!

В Rust также есть шаблон, который можно использовать, когда мы не хотим использовать значение в универсальном шаблоне: `_`, который является специальным шаблоном, который соответствует любому значению и не привязывается к этому значению. Это говорит Rust, что мы не собираемся использовать это значение, поэтому Rust не будет предупреждать нас о неиспользуемой переменной.

Давайте изменим правила игры так: если выпадает что-то, кроме 3 или 7, нужно бросить ещё раз. Нам не нужно использовать значение в этом случае, поэтому мы можем изменить наш код, чтобы использовать `_` вместо переменной с именем `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Этот пример также удовлетворяет требованию исчерпывающей полноты, поскольку мы явно игнорируем все остальные значения в последней ветке; мы ничего не забыли.

Finally, we’ll change the rules of the game one more time so that nothing else happens on your turn if you roll anything other than a 3 or a 7. We can express that by using the unit value (the empty tuple type we mentioned in [“The Tuple Type”](ch03-02-data-types.html#the-tuple-type)<!-- ignore --> section) as the code that goes with the `_` arm:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Здесь мы явно говорим Rust, что не собираемся использовать никакое другое значение, которое не соответствует шаблонам в предыдущих ветках, и не хотим запускать никакой код в этом случае.

Подробнее о шаблонах и совпадениях мы поговорим в [Главе 18](ch18-00-patterns.html)<!-- ignore -->. Пока же мы перейдём к синтаксису `if let`, который может быть полезен в ситуациях, когда выражение `match` слишком многословно.
