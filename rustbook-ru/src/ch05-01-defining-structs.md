## Определение и инициализация структур

Внешне структуры похожи на кортежи. Также как кортежи, структуры могут содержать
разные типы данных. Но в отличии от кортежей, все данные должны быть именованными.
Поэтому структуры более удобны для создания новых типов данных, т.к. нет необходимости
запоминать порядковый номер какого-либо значения внутри экземпляра структуры.

Для определения структуры, необходимо указать ключевое слово `struct` и имя.
Имя должно описывать содержание. Далее, в фигурных скобках, через запятую, вписывается
именованный состав данного типа данных. Каждый элемент, *поле*, имеет тип данных.
Пример 5-1, описывает структуру для хранения информации о учётной записи пользователя:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Пример 5-1: определение структуры <code>User</code></span>

После определения структуры можно создавать её *экземпляры*. Для этого каждому полю
определяется конкретное значение, соответствующее типу данных. Мы создаём экземпляр
указывая его имя и далее, в фигурных скобках, вписываем вместо типа данных конкретные
данные. Нет необходимости чётко следовать порядку следования полей (но всё-таки
желательно, для удобства чтения). Структура - это шаблон, а экземпляр - это шаблон
с данными. Пример 5-2:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

<span class="caption">Листинг 5-2: Создание экземпляра структуры <code>User</code></span>

Чтобы получить какое-нибудь значения поля структуры, мы можем использовать
точечную нотацию (как в кортеже). Например: `user1.email`. Для изменения значения
данных поля структуры (если оно изменяемое), мы просто присваиваем ему новое значение.
Пример 5-3:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

<span class="caption">Листинг 5-3: Изменение значения поля <code>email</code> экземпляра структуры <code>User</code></span>

Заметим, что весь экземпляр структуры должен быть изменяемым; Rust не позволяет помечать изменяемыми отдельные поля. Как в любом выражением, можно создавать новый экземпляр структуры в качестве последнего выражения тела функции для явного возврата нового экземпляра.

Listing 5-4 shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value of `true`, and
the `sign_in_count` gets a value of `1`.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Пример 5-4: Функция <code>build_user</code> имеющая две входные переменные</span>

Если имя переменной функции и поля структуры повторяется, то можно не писать повторяющиеся
наименования:

### Использование сокращения инициализации полей, когда переменные и имена  совпадают

Because the parameter names and the struct field names are exactly the same in
Listing 5-4, we can use the *field init shorthand* syntax to rewrite
`build_user` so that it behaves exactly the same but doesn’t have the
repetition of `email` and `username`, as shown in Listing 5-5.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Листинг 5-5: Функция <code>build_user</code> использует сокращение инициализации полей, потому что параметры <code>email</code> и <code>username</code> имеют имена как поля структуры</span>

Here, we’re creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field’s value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.

### Создание экземпляра структуры из экземпляра другой структуры с помощью синтаксиса обновления структуры

Часто является полезным создание нового экземпляра структуры, который использует значения старого экземпляра, но что-то меняет в нем. Это делается с помощью *синтаксиса обновления структур*.

Часто бывает удобно создавать новый экземпляр на основе старого. Пример 5-6 показывает
пример создания нового экземпляра на основе старого:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

<span class="caption">Пример 5-6: Создание экземпляра <code>User</code> присвоением  полям значений из <code>user1</code></span>

Используя синтаксис обновления структуры, можно получить тот же эффект, используя меньше кода как показано в листинге 5-7. Синтаксис `..` указывает, что оставшиеся поля устанавливаются не явно и должны иметь значения из указанного экземпляра.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

<span class="caption">Листинг 5-7: Использование синтаксиса обновления структур для установки значений <code>email</code> и <code>username</code> экземпляра <code>User</code>, но использование остальных значений из полей экземпляра переменной <code>user1</code></span>

Код в листинге Listing 5-7 также создает экземпляр переменной  `user2`, который имеет другое значение поля `email` и `username`, но те же значение для полей `active` и `sign_in_count` что у переменной `user1`.

### Использование кортеж структур без именованных полей для создания разных типов

Можно определять структуры с помощью сокращенной записи, очень напоминающей кортежи (такое определение называют *кортежными структурами*).  Кортежная структура имеет дополнительное значение из-за имени в объявлении, но не имеет имен ассоциированных с ее полями, у них есть только типы. Кортежные  структуры удобны, когда нужно дать всему кортежу имя и сделать его отличным от других кортежей, при этом именование полей как у обычной структуры будет подробным или избыточным.

Определение кортежной структуры начинается ключевым словом `struct`, названием структуры за которым следуют типы в кортеже. Например, вот определение и использование двух кортежных структур с именами `Color` и `Point`:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Обратите внимание, что переменные `black` и `origin` имеют разные типы данных.
Обращение к полям структур осуществляется с помощью `.` точечной нотации.

### Единичные структуры без полей

Можно также определять структуры без полей! Они называются  *unit-like, единично-подобные структуры* потому что ведут себя подобно единичному типу `()`. Единично-подобные структуры могут быть полезны в ситуации, в которой нужно реализовать типаж некоторого типа, но нет никаких данных для сохранения в самом типе. Мы обсудим типажи в главе 10.

> ### Владение данными структуры
> При определении структуры `User` в листинге  5-1 мы использовали владеющий тип `String`
> вместо `&str`. Это было осознанное решение, т.к. мы хотели, чтобы экземпляры структур владели всеми своими данными и чтобы данные были действительными во время всего существования структуры.
> Возможно так, чтобы структуры сохраняли ссылки на данные которыми владеет кто-то другой, но это требует использования *времен жизни*, особенности Rust о которой мы поговорим в главе 10. Время жизни гарантирует, что данные на которые ссылается структура, действительны столько же, сколько действительна сама структура. Допустим, вы пробуете сохранить ссылку в структуре без указания времени жизни, вот так, но это не работает:
> <span class="filename">Файл: src/main.rs</span>
> ```rust,ignore,does_not_compile
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
> 
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
> Компилятор будет жаловаться на необходимость определения времени жизни:
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
> 
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
> Мы расскажем, как исправить такие ошибки сохранения ссылок в структурах в главе 10. Чтобы исправить эту ошибку сейчас, с помощью имеющегося у Вас багажа знаний по Rust, используйте тип `String` вместо `&str`.
