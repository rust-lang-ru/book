## Дополнение Б: Операторы и обозначения

Это дополнение содержит глоссарий синтаксиса Rust, включая операторы и
другие обозначения, которые появляются сами по себе или в контексте путей, обобщений,
типажей, макросов, атрибутов, комментариев, кортежей и скобок.

### Операторы

Таблица Б-1 содержит операторы языка Rust, пример появления оператора, короткое объяснение, возможность перегрузки оператора. Если оператор можно перегрузить, то показан типаж, с помощью которого его можно перегрузить.

<span class="caption">Таблица Б-1: Операторы</span>

Оператор | Пример | Объяснение | Перегружаемость
--- | --- | --- | ---
`!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Вызов макроса | 
`!` | `!expr` | Побитовое или логическое отрицание | `Not`
`!=` | `var != expr` | Сравнение "не равно" | `PartialEq`
`%` | `expr % expr` | Остаток от деления | `Rem`
`%=` | `var %= expr` | Остаток от деления и присваивание | `RemAssign`
`&` | `&expr`, `&mut expr` | Заимствование | 
`&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Указывает что данный тип заимствуется | 
`&` | `expr & expr` | Bitwise AND | `BitAnd`
`&=` | `var &= expr` | Bitwise AND and assignment | `BitAndAssign`
`&&` | `expr && expr` | Logical AND | 
`*` | `expr * expr` | Arithmetic multiplication | `Mul`
`*=` | `var *= expr` | Arithmetic multiplication and assignment | `MulAssign`
`*` | `*expr` | Разыменование ссылки | 
`*` | `*const type`, `*mut type` | Указывает, что данный тип является сырым указателем | 
`+` | `trait + trait`, `'a + trait` | Соединение ограничений типа | 
`+` | `expr + expr` | Arithmetic addition | `Add`
`+=` | `var += expr` | Arithmetic addition and assignment | `AddAssign`
`,` | `expr, expr` | Argument and element separator | 
`-` | `- expr` | Arithmetic negation | `Neg`
`-` | `expr - expr` | Arithmetic subtraction | `Sub`
`-=` | `var -= expr` | Arithmetic subtraction and assignment | `SubAssign`
`->` | `fn(...) -> type`, <code>|...| -> type</code> | Указывает возвращаемый тип функции и замыкания | 
`.` | `expr.ident` | Доступ к элементу | 
`..` | `..`, `expr..`, `..expr`, `expr..expr` | Указывает на диапазон чисел, исключая правый | 
`..=` | `..=expr`, `expr..=expr` | Указывает на диапазон чисел, включая правый | 
`..` | `..expr` | Синтаксис обновления структуры | 
`..` | `variant(x, ..)`, `struct_type { x, .. }` | “And the rest” pattern binding | 
`...` | `expr...expr` | В шаблоне: шаблон диапазона включая правый элемент | 
`/` | `expr / expr` | Arithmetic division | `Div`
`/=` | `var /= expr` | Арифметическое деление и присваивание | `DivAssign`
`:` | `pat: type`, `ident: type` | Ограничения типов | 
`:` | `ident: expr` | Инициализация поля структуры | 
`:` | `'a: loop {...}` | Метка цикла | 
`;` | `expr;` | Оператор, указывающий на конец высказывания | 
`;` | `[...; len]` | Part of fixed-size array syntax | 
`<<` | `expr << expr` | Битовый сдвиг влево | `Shl`
`<<=` | `var <<= expr` | Битовый сдвиг влево и присваивание | `ShlAssign`
`<` | `expr < expr` | Сравнение "меньше чем" | `PartialOrd`
`<=` | `expr <= expr` | Сравнение "меньше или равно" | `PartialOrd`
`=` | `var = expr`, `ident = type` | Присваивание/эквивалентность | 
`==` | `expr == expr` | Сравнение "равно" | `PartialEq`
`=>` | `pat => expr` | Часть синтаксиса конструкции match | 
`>` | `expr > expr` | Сравнение "больше чем" | `PartialOrd`
`>=` | `expr >= expr` | Сравнение "больше или равно" | `PartialOrd`
`>>` | `expr >> expr` | Битовый сдвиг вправо | `Shr`
`>>=` | `var >>= expr` | Битовый сдвиг вправо и присваивание | `ShrAssign`
`@` | `ident @ pat` | Pattern binding | 
`^` | `expr ^ expr` | Bitwise exclusive OR | `BitXor`
`^=` | `var ^= expr` | Побитовое исключающее ИЛИ и присваивание | `BitXorAssign`
<code>|</code> | <code>pat | pat</code> | Pattern alternatives | 
<code>|</code> | <code>expr | expr</code> | Побитовое ИЛИ | `BitOr`
<code>|=</code> | <code>var |= expr</code> | Побитовое ИЛИ и присваивание | `BitOrAssign`
<code>||</code> | <code>expr || expr</code> | Логическое ИЛИ | 
`?` | `expr?` | Возврат ошибки | 

### Обозначения не-операторы

Следующий список содержит все не-литералы, которые не являются операторами.
То есть они не ведут себя как вызов функции или метода.

Table B-2 shows symbols that appear on their own and are valid in a variety of
locations.

<span class="caption">Таблица Б-2: Автономный синтаксис</span>

Обозначение | Объяснение
--- | ---
`'ident` | Именованное время жизни или метка цикла
`...u8`, `...i32`, `...f64`, `...usize`, etc. | Numeric literal of specific type
`"..."` | Строковый литерал
`r"..."`, `r#"..."#`, `r##"..."##`, etc. | Необработанный строковый литерал, в котором не обрабатываются escape-символы
`b"..."` | Строковый байтовый литерал; создает `[u8]` вместо строки
`br"..."`, `br#"..."#`, `br##"..."##`, etc. | Необработанный строковый байтовый литерал, комбинация необработанного и байтового литерала
`'...'` | Символьный литерал
`b'...'` | ASCII байтовый литерал
<code>|...| expr</code> | Замыкание
`!` | Always empty bottom type for diverging functions
`_` | «Игнорируемое» связывание шаблонов; также используется для читабельности целочисленных литералов

Таблица Б-3 показывает обозначения которые появляются в контексте путей иерархии модулей

<span class="caption">Таблица Б-3. Синтаксис, связанный с путями</span>

Обозначение | Объяснение
--- | ---
`ident::ident` | Namespace path
`::path` | Путь относительно корня крейта (т. е. явный абсолютный путь)
`self::path` | Путь относительно текущего модуля (т. е. явный относительный путь).
`super::path` | Путь относительно родительского модуля текущего модуля
`type::ident`, `<type as trait>::ident` | Ассоциированные константы, функции и типы
`<type>::...` | Ассоциированный элемент для типа, который не может быть назван прямо (например `<&T>::...`, `<[T]>::...`, etc.)
`trait::method(...)` | Устранение неоднозначности вызова метода путем именования типажа, который определяет его
`type::method(...)` | Disambiguating a method call by naming the type for which it’s defined
`<type as trait>::method(...)` | Устранение неоднозначности вызова метода путем именования типажа и типа

Table B-4 shows symbols that appear in the context of using generic type
parameters.

<span class="caption">Таблица Б-4: Обобщения</span>

Обозначение | Объяснение
--- | ---
`path<...>` | Specifies parameters to generic type in a type (e.g., `Vec<u8>`)
`path::<...>`, `method::<...>` | Specifies parameters to generic type, function, or method in an expression; often referred to as turbofish (e.g., `"42".parse::<i32>()`)
`fn ident<...> ...` | Define generic function
`struct ident<...> ...` | Определение обобщенной структуры
`enum ident<...> ...` | Define generic enumeration
`impl<...> ...` | Определение обобщенной реализации
`for<...> type` | Высокоуровневое связывание времени жизни
`type<ident=type>` | A generic type where one or more associated types have specific assignments (e.g., `Iterator<Item=T>`)

Table B-5 shows symbols that appear in the context of constraining generic type
parameters with trait bounds.

<span class="caption">Таблица Б-5: Ограничения типов</span>

Обозначение | Объяснение
--- | ---
`T: U` | Generic parameter `T` constrained to types that implement `U`
`T: 'a` | Generic type `T` must outlive lifetime `'a` (meaning the type cannot transitively contain any references with lifetimes shorter than `'a`)
`T : 'static` | Generic type `T` contains no borrowed references other than `'static` ones
`'b: 'a` | Generic lifetime `'b` must outlive lifetime `'a`
`T: ?Sized` | Позволяет обобщенным типам параметра иметь динамический размер
`'a + trait`, `trait + trait` | Соединение ограничений типов

Таблица Б-6 показывает обозначения, которые появляются в контексте вызова или определения макросов и указания атрибутов элемента.

<span class="caption">Таблица Б-6: Макросы и атрибуты</span>

Обозначение | Объяснение
--- | ---
`#[meta]` | Outer attribute
`#![meta]` | Inner attribute
`$ident` | Подстановка в макросе
`$ident:kind` | Macro capture
`$(…)…` | Macro repetition

Таблица Б-7 показывает обозначения, которые создают комментарии.

<span class="caption">Таблица Б-7: Комментарии</span>

Обозначение | Объяснение
--- | ---
`//` | Однострочный комментарий
`//!` | Внутренний однострочный комментарий документации
`///` | Внешний однострочный комментарий документации
`/*...*/` | Многострочный комментарий
`/*!...*/` | Внутренний многострочный комментарий документации
`/**...*/` | Внешний многострочный комментарий документации

Таблица Б-8 показывает обозначения, которые появляются в контексте использования кортежей.

<span class="caption">Таблица Б-8: Кортежи</span>

Обозначение | Объяснение
--- | ---
`()` | Пустой кортеж, он же пустой тип. И литерал и тип.
`(expr)` | Parenthesized expression
`(expr,)` | Кортеж с одним элементом выражения
`(type,)` | Кортеж с одним элементом типа
`(expr, ...)` | Tuple expression
`(type, ...)` | Tuple type
`expr(expr, ...)` | Выражение вызова функции; также используется для инициализации структур-кортежей и вариантов-кортежей перечисления
`ident!(...)`, `ident!{...}`, `ident![...]` | Вызов макроса
`expr.0`, `expr.1`, etc. | Взятие элемента по индексу в кортеже

Таблица Б-9 показывает контексты, в которых используются фигурные скобки.

<span class="caption">Таблица Б-9: Фигурные скобки</span>

Контекст | Объяснение
--- | ---
`{...}` | Выражение блока
`Type {...}` | `struct` литерал

Таблица Б-10 показывает контексты, в которых используются квадратные скобки.

<span class="caption">Таблица Б-10: Квадратные скобки</span>

Контекст | Объяснение
--- | ---
`[...]` | Литерал массива
`[expr; len]` | Литерал массива, содержащий `len` копий `expr`
`[type; len]` | Массив, содержащий `len` экземпляров типа `type`
`expr[expr]` | Взятие по индексу в коллекции. Возможна перегрузка (`Index`, `IndexMut`)
`expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Взятие среза коллекции по индексу, используется `Range`, `RangeFrom`, `RangeTo`, или `RangeFull` как "индекс"
