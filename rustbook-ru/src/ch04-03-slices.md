## Срезы

*Срезы (slices)*  - это ссылочный тип, не использующий владение.
Это непрерывная коллекция упорядоченных элементов.

Рассмотрим учебную задачу. Необходимо написать функцию, входным параметром которой
является строка. Выходным значением функции является первое слово, которое будет
найдено в этой строке. Если функция не находит разделителя слов (пробела), она
возвращает эту строку.

Прежде всего рассмотрим описание этой функции:

```rust,ignore
fn first_word(s: &String) -> ?
```

Функция `first_word` имеет входной параметр типа `&String`. Нам не нужно владение переменной, так что это нормально. Но что мы должны вернуть? На самом деле у нас нет способа говорить о *части* строки. Тем не менее, для решения задачи мы можем найти индекс конца слова в строке. Попробуем сделать как на листинге 4-7:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let index = first_word(&String::from("hello, Nik!"));
    println!("{}", index);
}

```

<span class="caption">Listing 4-10: Пример функции <code>first_word</code>, которая возвращает
index пробела в строке типа <code>String</code></span>

Для того, чтобы найти пробел в строке, мы превратим `String` в массив байт, используя метод `as_bytes` и пройдем по `String` элемент за элементом, проверяя является ли значение пробелом.

```rust,ignore
let bytes = s.as_bytes();
```

Далее, используя метод массива `iter()` мы создаём объект для последовательного
перебора содержания массива - итератор. Далее, используя цикл `for`, мы перебираем
байты и анализируем каждый из них. Обратите внимание, что при каждой итерации мы
получаем индекс элемента и ссылку на него:

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

Мы изучим итераторы более детально в главе 13. Сейчас, достаточно понять, что метод `iter` возвращает каждый элемент коллекции, а метод `enumerate` оборачивает результаты работы метода `iter` и возвращает каждый элемент как кортеж. Первый элемент этого кортежа возвращенный из  `enumerate` является индексом, а второй элемент - ссылкой на элемент. Такой способ перебора элементов массива является более удобным, чем считать индекс самостоятельно.

Так как метод `enumerate` возвращает кортеж, мы можем использовать шаблон деструктуризации кортежа, как и везде в Rust. Так в цикле `for`, мы указываем шаблон имеющий `i` для индекса в кортеже и `&item` для байта в кортеже. По причине получения ссылки на элемент из метода  `.iter().enumerate()`, используется `&` в шаблоне.

Внутри цикла `for`, ищем байт представляющий пробел используя синтаксис байт литерала. Если пробел найден, возвращается его позиция. Иначе, возвращается длина строки используя `s.len()`:

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

Таким образом, мы получаем искомое значение. Но оно может устареть в будущем  4-11:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return i;
#         }
#     }
#
#     s.len()
# }
#
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 'word' получит значение 5

    s.clear(); // очистка String, что делает содержимое равное ""

    // 'word' все ещё содержит 5, но уже нет строки с которой мы 
    // могли бы осмысленно использовать 5. значение word теперь полностью неверное!
}
```

<span class="caption">Listing 4-11: Сохранение результата вызова функции <code>first_word</code>,
потом изменяем содержимое переменной <code>s</code></span>

Данная программа компилируется без ошибок и возможно продолжит это делать, если мы воспользуемся `word` после вызова `s.clear()`. Так как значение `word` совсем не связано с состоянием переменной `s`, то `word` всё ещё имеет значение  `5`. Мы могли бы использовать `5` вместе с переменной `s` и попытаться извлечь первое слово, но это будет ошибкой, потому что содержимое `s` изменилось после того как мы сохранили `5` в переменной `word`.

Необходимость беспокоиться о том, что индекс в переменной `word` не синхронизируется с данными в переменной  `s` является утомительной и подверженной ошибкам! Управление этими индексами становится еще более хрупким, если мы напишем функцию `second_word`. Ее сигнатура могла бы выглядеть так:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Обратите внимание, что весьма сложно удерживать в синхронном состоянии вcе эти переменные
(входящие и исходящие). Для этих целей существуют срезы.

Luckily, Rust has a solution to this problem: string slices.

### Строковые срезы

Строковые срезы - это ссылка на часть строки `String`, и её инициализация
выглядит следующим образом:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

Эта инициализация похожа на создание ссылки на переменную `String`, но с дополнительным условием - указанием отрезка `[0..5]`. Вместо ссылки на всю `String`, срез ссылается на её часть.

Мы можем создавать срезы, используя определение отрезка `[starting_index..ending_index]`, где `starting_index` означает первую позицию в срезе, а `ending_index` на единицу больше, чем последняя позиция. Во внутреннем представлении, структура данный срез хранит начальную позицию и длину среза, которая соответствует числу `ending_index` минус `starting_index`. Таким образом, в примере `let world = &s[6..11];`, переменная `world` будет срезом, которая содержит ссылку на 7-ой байт в `s` со значением длины равным 5.

Рисунок 4-12 объясняет это схематично.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="../../rustbook-en/src/img/trpl04-06.svg" class="center" style="width: 50%;">

<span class="caption">Figure 4-12: Срез ссылается на часть
<code>String</code></span>

Синтаксис Rust позволяет упростить описание среза, если он начинается
с 0-го индекса:

```rust
fn main(){
    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{}",slice);
    let slice = &s[..2];
    println!("{}",slice);
}
```

Таким же образом можно поступить с последним элементом, если это последний байт в
`String`:

```rust
fn main() {
    let s = String::from("hello");

    let len = s.len();
    println!("sting length = {}", len);
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);
}

```

Таким образом, срез целой строки можно описать так:

```rust
fn main() {
    let s = String::from("hello");
    println!("{}", s);
    let len = s.len();
    println!("a length of the string = {}", s);
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}

```

> Внимание: Индексы среза строк должны соответствовать границам UTF-8 символов. Если вы попытаетесь получить срез нарушая границы мультибайтового символа, то вы получите ошибку времени исполнения. В рамках этой главы мы будем предполагать только ASCII кодировку. Более детальное обсуждение UTF-8 находится в секции  [“Сохранение текста с кодировкой UTF-8 в строках”](ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)<comment> главы 8.</comment>

Применим полученные знания и перепишем метод `first_word`. Для обозначения типа
"срез строки" существует запись `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Теперь, вызвав метод `first_word`, мы получим один объект, который включает в себя
всю необходимую информацию.

Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.

Аналогичным образом можно переписать и второй метод `second_word`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

Благодаря использованию срезов нельзя изменить данные строки, если
на неё ссылается срез (т.к. это может привести к ошибке):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Ошибка компиляции:

```text
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

Благодаря соблюдению правил, Rust просто исключает класс подобных ошибок.

#### Строковые константы - срезы

Вооружившись знаниями о срезах можно посмотреть по-новому на
инициализацию переменной строкового типа:

```rust
let s = "Hello, world!";
```

Тип `s` является `&str` - это срез, указывающий на конкретное значение в коде программы.
Поэтому строковый литерал неизменяемый, а тип `&str` это неизменяемая ссылка.

#### Строковые срезы как параметры

Используя строковые срезы, как параметры вы можете улучшить
код ваших методов:

```rust,ignore
fn first_word(s: &String) -> &str {
```

Также можно записать этот код следующим образом:

```rust,ignore
fn first_word(s: &str) -> &str {
```

<span class="caption">Пример 4-9: Улучшение функции <code>first_word</code> используя тип строкового среза для параметра <code>s</code></span>

Если мы используем срез, мы можем передавать его в методы.
Использование срезов вместо переменных делает код более удобным:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &str) -> &str {
   let bytes = s.as_bytes();
   for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
           return &s[0..i];
       }
     }
   &s[..]
}
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("{}",word);
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{}",word);
    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}",word);
}
```

### Другие срезы

Существуют также срезы других типов. Рассмотрим массив:

```rust
let a = [1, 2, 3, 4, 5];
```

Создадим срез:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

Этот срез имеет тип данных `&[i32]`. Мы поговорим о таком типе
коллекций в главе 8.

## Итоги

Такие концепции как владение, заимствование и срезы - это способы
защиты использования памяти.  Rust даёт вам возможность контролировать использование
памяти.

Владение влияет на множество других концепций языка Rust.
В следующей главе мы рассмотрим способ группировки данных в `struct`.
