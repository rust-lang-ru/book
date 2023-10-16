## Определение перечисления

Там, где структуры дают вам возможность группировать связанные поля и данные, например `Rectangle` с его `width` и `height`, перечисления дают вам способ сказать, что значение является одним из возможных наборов значений. Например, мы можем захотеть сказать, что `Rectangle` — это одна из множества возможных фигур, в которую также входят `Circle` и `Triangle`. Для этого Rust позволяет нам закодировать эти возможности в виде перечисления.

Давайте рассмотрим ситуацию, которую мы могли бы захотеть отразить в коде, и поймём, почему перечисления полезны и более уместны, чем структуры в этом случае. Допустим, нам нужно работать с IP-адресами. В настоящее время для обозначения IP-адресов используются два основных стандарта: четвёртая и шестая версии. Поскольку это единственно возможные варианты IP-адресов, с которыми может столкнуться наша программа, мы можем *перечислить* все возможные варианты, откуда перечисление и получило своё название.

Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

Можно выразить эту концепцию в коде, определив перечисление `IpAddrKind` и составив список возможных видов IP-адресов, `V4` и `V6`. Вот варианты перечислений:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` теперь является пользовательским типом данных, который мы можем использовать в другом месте нашего кода.

### Значения перечислений

Экземпляры каждого варианта перечисления `IpAddrKind` можно создать следующим образом:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Обратите внимание, что варианты перечисления находятся в пространстве имён вместе с его идентификатором, а для их обособления мы используем двойное двоеточие. Это удобно тем, что теперь оба значения `IpAddrKind::V4` и `IpAddrKind::V6` относятся к одному типу: `IpAddrKind`. Затем мы можем, например, определить функцию, которая принимает любой из вариантов `IpAddrKind`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Можно вызвать эту функцию с любым из вариантов:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Использование перечислений позволяет получить ещё больше преимуществ. Если подумать о нашем типе для IP-адреса, то выяснится, что на данный момент у нас нет возможности хранить собственно сам *IP-адрес*; мы будем знать только его *тип*. Учитывая, что недавно в главе 5 вы узнали о структурах, у вас может возникнуть соблазн решить эту проблему с помощью структур, как показано в листинге 6-1.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Листинг 6-1. Сохранение данных и <code>IpAddrKind</code> IP-адреса с использованием <code>struct</code></span>

Здесь мы определили структуру `IpAddr`, у которой есть два поля: `kind` типа `IpAddrKind` (перечисление, которое мы определили ранее) и `address` типа `String`. У нас есть два экземпляра этой структуры. Первый - `home`, который является `IpAddrKind::V4` в качестве значения `kind` с соответствующим адресом `127.0.0.1`. Второй экземпляр - `loopback`. Он в качестве значения `kind` имеет другой вариант `IpAddrKind`, `V6`, и с ним ассоциирован адрес `::1`. Мы использовали структуру для объединения значений `kind` и `address` вместе, таким образом тип формата адреса теперь ассоциирован со значением.

Однако представление этой же концепции с помощью перечисления более лаконично: вместо того, чтобы помещать перечисление в структуру, мы можем поместить данные непосредственно в любой из вариантов перечисления. Это новое определение перечисления `IpAddr` гласит, что оба варианта `V4` и `V6` будут иметь соответствующие значения `String`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

We attach data to each variant of the enum directly, so there is no need for an extra struct. Here, it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum. That is, `IpAddr::V4()` is a function call that takes a `String` argument and returns an instance of the `IpAddr` type. We automatically get this constructor function defined as a result of defining the enum.

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. Version four IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but still express `V6` addresses as one `String` value, we wouldn’t be able to with a struct. Enums handle this case with ease:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Мы показали несколько различных способов определения структур данных для хранения IP-адресов четвёртой и шестой версий. Однако, как оказалось, желание хранить IP-адреса и указывать их тип настолько распространено, что в стандартной библиотеке есть определение, которое мы можем использовать!<!-- ignore --> Давайте посмотрим, как стандартная библиотека определяет `IpAddr`: в ней есть точно такое же перечисление с вариантами, которое мы определили и использовали, но она помещает данные об адресе внутрь этих вариантов в виде двух различных структур, которые имеют различные определения для каждого из вариантов:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Этот код иллюстрирует что мы можем добавлять любой тип данных в значение перечисления: строку, число, структуру и пр. Вы даже можете включить в перечисление другие перечисления! Стандартные типы данных не очень сложны, хотя, потенциально, могут быть очень сложными (вложенность данных может быть очень глубокой).

Обратите внимание, что хотя определение перечисления `IpAddr` есть в стандартной библиотеке, мы смогли объявлять и использовать свою собственную реализацию с аналогичным названием без каких-либо конфликтов, потому что мы не добавили определение стандартной библиотеки в область видимости кода. Подробнее об этом поговорим в Главе 7.

Рассмотрим другой пример перечисления в листинге 6-2: в этом примере каждый элемент перечисления имеет свой особый тип данных внутри:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Листинг 6-2. Перечисление <code>Message</code>, в каждом из вариантов которого хранятся разные количества и типы значений.</span>

Это перечисление имеет 4 элемента:

- `Quit` - пустой элемент без ассоциированных данных,
- `Move` has named fields, like a struct does.
- `Write` - элемент с единственной строкой типа `String`,
- `ChangeColor` - кортеж из трёх значений типа `i32`.

Определение перечисления с вариантами, такими как в листинге 6-2, похоже на определение значений различных типов внутри структур, за исключением того, что перечисление не использует ключевое слово `struct` и все варианты сгруппированы внутри типа `Message`. Следующие структуры могут содержать те же данные, что и предыдущие варианты перечислений:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the `Message` enum defined in Listing 6-2, which is a single type.

Есть ещё одно сходство между перечислениями и структурами: так же, как мы можем определять методы для структур с помощью `impl` блока, мы можем определять и методы для перечисления. Вот пример метода с именем `call`, который мы могли бы определить в нашем перечислении `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

В теле метода будет использоваться `self` для получения значение того объекта, у которого мы вызвали этот метод. В этом примере мы создали переменную `m`, содержащую значение `Message::Write(String::from("hello"))`, и именно это значение будет представлять `self` в теле метода `call` при выполнении `m.call()`.

Теперь посмотрим на другое наиболее часто используемое перечисление из стандартной библиотеки, которое является очень распространённым и полезным: `Option`.

### Перечисление `Option` и его преимущества перед Null-значениями

В этом разделе рассматривается пример использования `Option`, ещё одного перечисления, определённого в стандартной библиотеке. Тип `Option` кодирует очень распространённый сценарий, в котором значение может быть чем-то, а может быть ничем.

For example, if you request the first item in a non-empty list, you would get a value. If you request the first item in an empty list, you would get nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.

Дизайн языка программирования часто рассматривается с точки зрения того, какие функции вы включаете в него, но те функции, которые вы исключаете, также важны. Например в Rust нет такого функционала как null значения, однако он есть во многих других языках. *Null значение* - это значение, которое означает, что значения нет. В языках с null значением переменные всегда могут находиться в одном из двух состояний: *нет значения (null)* или *есть значение (not-null)*.

В своей презентации 2009 года «Null ссылки: ошибка в миллиард долларов» Тони Хоар (Tony Hoare), изобретатель null, сказал следующее:

> Я называю это своей ошибкой на миллиард долларов. В то время я разрабатывал первую комплексную систему типов для ссылок на объектно-ориентированном языке. Моя цель состояла в том, чтобы гарантировать, что любое использование ссылок должно быть абсолютно безопасным, с автоматической проверкой компилятором. Но я не мог устоять перед соблазном вставить пустую ссылку просто потому, что это было так легко реализовать. Это привело к бесчисленным ошибкам, уязвимостям и системным сбоям, которые, вероятно, причинили боль и ущерб на миллиард долларов за последние сорок лет.

Проблема с null значениями заключается в том, что если вы попытаетесь использовать null значение в качестве not-null значения, вы получите ошибку определённого рода. Поскольку свойство null или not-null распространено повсеместно, сделать такую ошибку очень просто.

Тем не менее, концепция, которую null пытается выразить, является полезной: null - это значение, которое в настоящее время по какой-то причине недействительно или отсутствует.

Проблема на самом деле не в концепции, а в конкретной реализации. Таким образом, в Rust значений null, но есть перечисление, которое может закодировать концепцию присутствия или отсутствия значения. Это перечисление `Option<T>` , и оно [определено стандартной библиотекой <!-- ignore --> следующим образом:](../std/option/enum.Option.html)

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Перечисление `Option<T>` настолько полезно, что оно даже включено в прелюдию; вам не нужно явно вводить его в область видимости. Его варианты также включены в прелюдию: вы можете использовать `Some` и `None` напрямую, без префикса `Option::`. При всём при этом, `Option<T>` является обычным перечислением, а `Some(T)` и `None` представляют собой его варианты.

The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter, and we’ll cover generics in more detail in Chapter 10. For now, all you need to know is that `<T>` means that the `Some` variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type a different type. Here are some examples of using `Option` values to hold number types and string types:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

Тип `some_number` - `Option<i32>`. Тип `some_char` - `Option<char>`, это другой тип. Rust может вывести эти типы, потому что мы указали значение внутри варианта `Some`. Для `absent_number` Rust требует, чтобы мы аннотировали общий тип для `Option`: компилятор не может вывести тип, который будет в `Some`, глядя только на значение `None`. Здесь мы сообщаем Rust, что `absent_number` должен иметь тип `Option<i32>`.

When we have a `Some` value, we know that a value is present and the value is held within the `Some`. When we have a `None` value, in some sense it means the same thing as null: we don’t have a valid value. So why is having `Option<T>` any better than having null?

In short, because `Option<T>` and `T` (where `T` can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value. For example, this code won’t compile, because it’s trying to add an `i8` to an `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

If we run this code, we get an error message like this one:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Сильно! Фактически, это сообщение об ошибке означает, что Rust не понимает, как сложить `i8` и `Option<i8>`, потому что это разные типы. Когда у нас есть значение типа на подобие `i8`, компилятор гарантирует, что у нас всегда есть допустимое значение типа. Мы можем уверенно продолжать работу, не проверяя его на null перед использованием. Однако, когда у нас есть значение типа `Option<T>` (где `T` - это любое значение любого типа `T`, упакованное в `Option`, например значение типа `i8` или `String`), мы должны беспокоиться о том, что <em>значение</em> типа T возможно не имеет значения (является вариантом `None`), и компилятор позаботится о том, чтобы мы обработали такой случай, прежде чем мы бы попытались использовать `None` значение.

Другими словами, вы должны преобразовать `Option<T>` в `T` прежде чем вы сможете выполнять операции с этим `T`. Как правило, это помогает выявить одну из наиболее распространённых проблем с null: предполагая, что что-то не равно null, когда оно на самом деле равно null.

Устранение риска ошибочного предположения касательно не-null значения помогает вам быть более уверенным в своём коде. Чтобы иметь значение, которое может быть null, вы должны явно описать тип этого значения с помощью `Option<T>`. Затем, когда вы используете это значение, вы обязаны явно обрабатывать случай, когда значение равно null. Везде, где значение имеет тип, отличный от `Option<T>`, вы *можете* смело рассчитывать на то, что значение не равно null. Это продуманное проектное решение в Rust, ограничивающее распространение null и увеличивающее безопасность кода на Rust.

So how do you get the `T` value out of a `Some` variant when you have a value of type `Option<T>` so that you can use that value? The `Option<T>` enum has a large number of methods that are useful in a variety of situations; you can check them out in [its documentation](../std/option/enum.Option.html)<!-- ignore -->. Becoming familiar with the methods on `Option<T>` will be extremely useful in your journey with Rust.

In general, in order to use an `Option<T>` value, you want to have code that will handle each variant. You want some code that will run only when you have a `Some(T)` value, and this code is allowed to use the inner `T`. You want some other code to run only if you have a `None` value, and that code doesn’t have a `T` value available. The `match` expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
