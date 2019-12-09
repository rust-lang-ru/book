## Выразительное управление с помощью `if let`

Конструкция `if let` позволяет комбинировать `if` и `let`, что позволяет присваивать
значение переменной, удовлетворяющее определённому шаблону. Пример 6-6:

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

<span class="caption">Пример 6-6. Будет напечатана строка, если some_u8_value будет
равна <code>Some(3)</code></span>

Мы хотим выполнить что-нибудь при совпадении `Some(3)` и ничего не делать с любым другим `Some<u8>` или значением `None` . Для удовлетворения `match` выражения необходимо добавить `_ => ()` после обработки только одного варианта, который экономит добавление массы стандартного кода.

Instead, we could write this in a shorter way using `if let`. The following
code behaves the same as the `match` in Listing 6-6:

```rust
# let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

The syntax `if let` takes a pattern and an expression separated by an equal
sign. It works the same way as a `match`, where the expression is given to the
`match` and the pattern is its first arm.

Using `if let` means less typing, less indentation, and less boilerplate code.
However, you lose the exhaustive checking that `match` enforces. Choosing
between `match` and `if let` depends on what you’re doing in your particular
situation and whether gaining conciseness is an appropriate trade-off for
losing exhaustive checking.

Другими словами, можно думать о `if let` как о синтаксическом сахаре для выражения `match` , которое выполняет код, когда значение соответствует одному шаблону и затем игнорирует все остальные значения.

We can include an `else` with an `if let`. The block of code that goes with the
`else` is the same as the block of code that would go with the `_` case in the
`match` expression that is equivalent to the `if let` and `else`. Recall the
`Coin` enum definition in Listing 6-4, where the `Quarter` variant also held a
`UsState` value. If we wanted to count all non-quarter coins we see while also
announcing the state of the quarters, we could do that with a `match`
expression like this:

```rust
# #[derive(Debug)]
# enum UsState {
#    Alabama,
#    Alaska,
# }
#
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter(UsState),
# }
# let coin = Coin::Penny;
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Или мы могли бы использовать выражение `if let` и `else` так:

```rust
# #[derive(Debug)]
# enum UsState {
#    Alabama,
#    Alaska,
# }
#
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter(UsState),
# }
# let coin = Coin::Penny;
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

Вам выбирать какая конструкция подходит для вашего кода лучше всего.

## Итоги

В этой главе мы рассмотрели, как использовать перечисление (создание, примеры использования).
Также, на примере типа из стандартной библиотеки `Option<T>`, мы выяснили, как
предотвратить ошибки в коде.  Изучили использование конструкций `match` и `if let`
для анализа и выборки данных из значений перечислений, а также некоторые возможные
улучшения и упрощения кода.

Теперь ваши программы Rust могут выражать концепции в предметной области используя структуры и перечисления. Создание пользовательских типов для использования в API обеспечивает безопасность типов: компилятор позаботится о том, чтобы функции получали значения только того типа, которые ожидает функция.

Чтобы предоставить хорошо организованный API пользователям, необходимо использовать и показывать только то, что нужно пользователям, давайте теперь обратимся к модулям в Rust.
