## Обработка группы элементов с помощью итераторов

<!-- From reading on, it seems like an iterator is useless without the methods
we use it with --- I think this is an important point to make early, I did find
it difficult to know what an iterator actually was throughout, and how it
depends on these other methods. Can you add something to this effect? -->
<!-- Reiterating the need for a clear definition of an iterator here--it seems
like an item that's used in iteration rather than something that performs the
process of iteration itself, is that right? Like a counter passed from element
to element? Can we define this at the begin of the iterator section? -->
<!-- I've added an explanation along these lines, does this help? /Carol -->

Шаблон (или паттерн) проектирования Итератор позволяет выполнять какие-либо 
действия с последовательностью элементов по очереди. *Итератор* отвечает за 
логику итерации по каждому элементу в наборе и определяет, когда он 
заканчивается. При использовании итератора, нам не нужно переопределять эту
логику самостоятельно.

Итераторы в Rust являются *ленивыми* (*lazy*), то есть они ничего не делают, 
пока мы не вызовем методы, которые могут их использовать. Например, код в 
листинге 13-13 создает итератор для элементов в векторе `v1`, вызывая метод 
`iter`, определенный в `Vec`. Этот код сам по себе ничего полезного не делает:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

<span class="caption">Код 13-13: Создание итератора</span>

После создания итератора, можно использовать его различными способами. В 
листинге 3-6 мы использовали итератор с циклом `for` при выполнении кода для 
каждого элемента, хотя и упустили из виду то, что делал вызов iter до этого 
момента. Пример в листинге 13-14 отделяет создание итератора от его 
использования в цикле `for`. Он сохраняется в переменной `v1_iter`, и в этот 
момент итерирование не производится. Когда цикл `for` вызывается для `v1_iter`, 
каждый элемент из последовательности используется в одной итерации цикла, 
которая выводит каждое значение в консоль:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

<span class="caption">Код 13-14: использование итератора в цикле `for`</span>

В языках программирования, в которых нет итераторов, предоставляемых 
стандартными библиотеками, мы, скорее всего, напишем тот же функционал, 
используя переменную для индексации элементов в векторе начиная с нуля. Получая 
значение и увеличивая счётчик элементов в цикле, пока его значение не станет 
равным длине вектора. Итераторы позаботятся обо всей этой логике за нас, 
сокращая повторяющийся код, который мы должны были бы написать.
Кроме того, реализация итераторов дает нам большую гибкость в использовании 
одной и той же логики со многими различными типами последовательностей, а не 
только со структурами данных, которые можно индексировать как векторы. Давайте 
посмотрим, как же итераторы это делают.

### Типаж `Iterator` и метод `next`

Все итераторы реализуют типаж стандартной библиотеки `Iterator`. Так выглядит
его исходный код:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // исключены стандартные методы реализации
}
```

Обратите внимание на элементы синтаксиса, которые мы ещё не рассматривали.
`type Item` и `Self::Item`, которые определяют *ассоциированный тип* (*associated type*)
с этим типажом. Мы подробнее поговорим о них в Главе 19. Сейчас вам нужно знать,
что этот код требует от реализаций этого типажа определить тип `Item`. Этот тип
используется в методе `next`. Другими словами, тип `Item` будет являться типом
элемента, который возвращает итератор.

Метод `next` необходимо реализовать. Возвращаемое значение находится внутри `Some`.
Когда перебор элементов завершен, возвращается `None`. Мы можем вызвать метод
`next` непосредственно. Пример 13-15:

<span class="filename">Файл: src/lib.rs</span>

```rust,test_harness
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

<span class="caption">Код 13-15: Вызов метода итератора `next`</span>

Обратите внимание, что нам нужно сделать `v1_iter` изменяемой: вызов метода 
`next` изменяет состояние итератора, которое следит за тем, где он находится в последовательности. Другими словами, этот код *потребляет* (*consumes*) или 
использует итератор. Каждый вызов `next` съедает элемент из итератора. Нам не 
нужно было делать изменяемым `v1_iter`, когда мы использовали цикл `for`, 
потому что цикл `for` стал владельцем `v1_iter` и сделал изменяемым `v1_iter` 
неявно для нас.

Также следует учесть, что значения, которые мы получаем вызывая `next`, являются неизменяемыми ссылками на значения в векторе. Метод `iter` создает итератор для неизменяемых ссылок. Если мы хотим создать итератор, который получает во 
владение `v1` и возвращает собственные значения, мы можем вызвать `into_iter` 
вместо `iter`. Точно так же, если мы хотим перебрать изменяемые ссылки, мы можем вызвать `iter_mut` вместо `iter`.

### Методы типажа `Iterator`

<!-- Can you explain what it is you mean by "consumes" an iterator here? It
doesn't look like we do in this section, I think that's important to lay that
out clearly -->
<!-- This next paragraph doesn't give much away to me I'm afraid, not being
clear what we mean by *consume* at this point. Is a consuming adaptor like a
catalyst? -->
<!-- I hope this section addresses these comments you had /Carol -->

Типаж `Iterator` имеет ряд различных методов с реализациями по умолчанию, предоставленных нам стандартной библиотекой; Вы можете узнать больше об этих 
методах, заглянув в документацию API стандартной библиотеки для типажа 
`Iterator`. Некоторые из этих методов используют метод `next` внутри, поэтому мы должны имплементировать метод `next` при реализации типажа `Iterator`.

<!-- Is there somewhere they can learn about all the methods and what they do,
how to use them? This seems like a good sample example, and if we can broaden
it out that would be really helpful -->
<!-- I've moved this comment here since you made this comment on the last
version of this chapter right after a spot where we mentioned looking at the
standard library API documentation for the iterator trait, like we're now doing
in the above paragraph. That's where the reader should go to learn about
all the methods and what they do and how to use them. Can you elaborate on why
that wasn't clear in the previous version of the chapter? Is there a reason why
the standard library API documentation didn't sound like that place to go?
/Carol -->

Методы, вызывающие `next`, называются *потребляющими адаптерами* (*consuming adaptors*), поскольку их вызов использует итератор. Примером потребляющего 
адаптера является метод `sum`. Он становится владельцем итератора и перемещается 
по элементам, многократно вызывая `next`, тем самым потребляя итератор. 
Он выполняет итерацию для каждого элементу, и добавляет его к промежуточной 
сумме, возвращая итоговую сумму после завершения итерации. В листинге 13-16 есть тест, иллюстрирующий использование `sum`:

<span class="filename">Файл: src/lib.rs</span>

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

<span class="caption">Код 13-16: Вызов метода `sum` для получения суммы всех
элементов вектора</span>

Переменную `v1_iter` после вызова метода `sum` уже использовать нельзя.

### Методы типажа `Iterator` для создания других итераторов

Другим типом методов в типаже `Iterator` являются методы создающие другие 
итераторы. Эти методы называют адаптерами (*iterator adaptors*) и позволяют нам заменять итераторы на различные типы итераторов. Мы можем использовать цепочки вызовов таких адаптеров. Однако, поскольку все итераторы ленивы, мы должны 
вызвать один из методов адаптера потребителя, чтобы получить результат после 
вызова адаптеров итератора. В листинге 13-17 показан пример вызова метода 
адаптера итератора `map`, который принимает замыкание. `map` будет вызываться 
для каждого элемента, чтобы создать новый итератор, в котором каждый элемент из вектора будет увеличен на 1. Этот код вызовет предупреждение:

<span class="filename">Файл: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

<span class="caption">Код 13-17: Вызов итератора-адаптера `map` для создания
нового итератора</span>

Предупреждение:

```text
warning: unused result which must be used: iterator adaptors are lazy and do
nothing unless consumed
 --> src/main.rs:4:1
  |
4 | v1.iter().map(|x| x + 1);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
```

Код программы 13-17 ничего не делает, пока не будет вызвано другим элементом 
цепочки вызовов. Об этом сообщается компилятором при вызове этого кода: т.к. адаптеры итераторов работают только при внешнем использовании.

Для того чтобы исправить код и последовать рекомендациям компилятора, будем использовать
метод `collect` (который мы кратко представили в Главе 12). Этот метод использует
итератор для группировки результатов работы предыдущего метода в вектор. В примере
кода 13-18 мы группируем результаты работы метода `map` в вектор, который содержит
все элементы первоначального вектора при этом значение каждого числового элементам
 увеличено на 1:

<span class="filename">Файл: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

<span class="caption">Код 13-18: вызов метода `map` для создания нового итератора,
далее вызов метода `collect` для создания и использования нового итератора, чтобы
создать новый вектор с данными</span>

Т.к. `map` получает замыкание, мы можем применить любую операцию над содержимым.
Это прекрасный пример того, как использование замыканий позволяет улучшить поведение
итераторов (упростить обработку данных).


<!-- I'm not clear from this last sentence which part is iterating through each
element, iter or map? What is map actually doing?-->
<!--Ah, I'm afraid I completely failed to follow this. What is the second
iterator for? I'm still not clear on what map does, can you expand on this? It
seems crucial to using iterators. Map applies the iterator to each element,
which applies the closure?

Also, to generalize this discussion a bit, would you ever use iter without map?
-->
<!-- I hope this new breakdown/rearranging has cleared up these comments you
had on the last version of this chapter about the difference between
iter and map. I hope the added examples where we've used iter without map have
cleared up the last question. /Carol -->

### Использование замыканий для получения доступа к переменным среды при работе итераторов

Продолжим расширение наших знаний об совместном использовании замыканий и итераторов.
Рассмотрим пример использования замыканий для получения доступа к переменным
внешней среды и использования адаптера итераторов `filter`. Этот метод получает в
качестве параметра замыкание, применяет замыкание к каждому элементу и возвращается
булево значение. Если в результате работы кода замыкание возвращается значение `false`,
то данный элемент игнорируется при создание нового итератора. Код 13-19 демонстрирует
использование `filter` и замыкания, которое получает доступ к переменной `shoe_size`
при обработки коллекции данных структур `Shoe` для того, чтобы выбрать только те,
которые подходят под определенный размер:


<span class="filename">Файл: src/lib.rs</span>

```rust,test_harness
#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

<span class="caption">Код 13-19: использование метода `filter`, замыкания и переменной
функции `shoe_size`</span>

<!-- Will add wingdings in libreoffice /Carol -->

Функция `shoes_in_my_size` получает во владение вектор структур данных и числовое
значение. Данная функция возвращает вектор содержащий только структуры подходящие
под определенные критерии (в данном случае описания обуви определенного размера).
В теле функции мы вызываем метод `into_iter` для создания итератора, который получит
владение вектором. Далее вызываем метод `filter`, который применит к каждому элементу
вектора замыкание. Данное замыкание возвращает логическое значение результат сравнения
поля структуры с аргументом функции. В результате, метод `collect` объединит полученные
данные в вектор, который будет возвращен функцией в качестве выходных данных.

Тест показывает, что когда мы вызываем `shoes_in_my_size`, мы получаем только ту обувь, размер которой равен указанному нами значению.

### Реализация типажа `Iterator` для создания нового итератора

<!-- So it seems like we are creating a program with an iterator inside, is
that right? I assumed the whole thing we were making was an iterator at first,
which lead to a few confusions, can you lay it out up front? -->
<!-- I'm not sure what you mean here, can you elaborate on what the distinction
is to you between "a program with an iterator inside" and "whole thing we were
making was an iterator"? I don't understand what you mean by these terms so I'm
not sure how to clear this up. /Carol -->

Вы уже видели в примерах, как можно создать итератор вектора (с помощью вызовов
функций `iter`, `into_iter` или `iter_mut`). Мы также можем создать итераторы из
других типов коллекций стандартной библиотеки (например, `map`). Помимо этого мы
можем реализовать `Iterator` для обработки любых данных. Для этого необходимо
реализовать метод `next`. После этого мы можем использовать все методы типажа
`Iterator` (используя реализации самого типажа).


<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

Реализуемый нами итератор будет считать от одного до пяти. Для начала создадим
структуры для хранения значений. Далее напишем реализацию типажа `Iterator`

В коде 13-20 определение структуры `Counter` и реализации функции `new` для
создания экземпляра структуры `Counter`:

<span class="filename">Файл: src/lib.rs</span>

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

<span class="caption">Код 13-20: определения структуры `Counter` и функции `new`,
которая создаёт экземпляр структуры `Counter` с инициализированным значением `0` поля
`count`</span>

<!-- Could you add a filename here? I think that will help the reader keep
track of what they're working on. Can you also just sum up in a line what this
code has accomplished so far? I moved this down from above the code, if this
will do? -->
<!-- Done /Carol -->

Структура `Counter` имеет одно поле с именем `count`. Это поле содержит значение 
`u32`, которое будет отслеживать, где мы находимся в процессе итерации от 1 до 
5. Поле `count` является закрытым, поскольку мы хотим, чтобы реализация 
`Counter` контролировала его значение. Функция `new` обеспечивает желаемое 
поведение: всегда инициализировать новые экземпляры значением 0 в поле `count`.

<!-- Why define the new method, if it isn't necessary? Or is that what this
next line is telling us? -->
<!-- So does this code just initialize it with 0? Is that jat { count: 0 }
does?-->
<!-- I've rearranged to make this clearer /Carol -->

Далее, мы реализуем типаж `Iterator` для структуры `Counter`, определив тело 
метода `next`, указывая, что мы хотим получить при использовании этого 
итератора. Как это показано в листинге 13-21:

<span class="filename">Файл: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<span class="caption">Код 13-21: реализация типажа `Iterator` в структуре `Counter`</span>

<!-- I will add wingdings in libreoffice /Carol -->

Рассмотрим содержание кода реализации типажа подробнее. Мы установили тип `Item`
(тип выходных данных метода `next`) `u32`. Более подробно о ассоциированных типах
мы поговорим в Главе 19. Обратим теперь внимание на содержание реализации метода.
Мы хотим чтобы наш итератор добавлял единицу к текущему значению. Поэтому мы инициировали
поле `count` 0. Если значение этого поля меньше 6, функция `next` возвращает текущее
значение внутри `Some`. Если же это поле равно 6 или больше итератор вернёт `None`.

#### Пример использования итератора `Counter`

После того как метод `next` реализован, т.е. соблюдены все ограничения типажа
`Iterator` - мы получили реализацию итератора. Код 13-22 демонстрирует проверку
работы нашей реализации:

<span class="filename">Файл: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Iterator for Counter {
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         self.count += 1;
#
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

<span class="caption">Код 13-22: тестирования реализации метода `next`</span>

В этом тесте создаётся экземпляр структуры `Counter` - переменная `counter`.
Далее вызывается метод `next` и проверяется его выходные данные. Как и предполагалось,
метод возвращает числа от 1 до 5, а при последующих вызовах возвращает `None`.

<!-- So if I have this right, the first line creates a new Counter called
counter, and the rest of them merely call counter with next, store it in x, and
then print x? And we have to do that 5 times to get the 1-5 count? Phew, could
you wrap that up if indeed it is correct!) and sum up here? -->
<!-- I decided to change this into a test rather than printing out values, and
I added some summary text about what the test is doing. Is this clearer? /Carol
-->

#### Использование других методов типажа `Iterator`

Т.к. мы реализовали типаж `Iterator`, мы можем использовать все его доступные
методы, определенные стандартной библиотекой, поскольку все они используют 
возможности метода `next`.

<!-- So we can't just use these methods anyway? It seems like we did earlier,
but here we have to use next first, before we cam access these methods? -->
<!-- No, we don't have to *use* `next` before we can use the other methods, we
have to *define* `next` before we can use the other methods. I hope the various
rewordings and reworkings have made this clearer by this point. /Carol -->

<!-- below: once you've done what, defined a default implementation? Only then
can you use other adapters, is that what we're saying? And I'm still not clear
on what an adapter does/means, as opposed to a method, or consumer, at this
point. -->
<!-- I hope this comment has been cleared up too /Carol -->

Пример использования методов типажа, доступных её реализации (13-23):

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Counter {
#     fn new() -> Counter {
#         Counter { count: 0 }
#     }
# }
#
# impl Iterator for Counter {
#     // Our iterator will produce u32s
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         // increment our count. This is why we started at zero.
#         self.count += 1;
#
#         // check to see if we've finished counting or not.
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```

<span class="caption">Listing 13-23: Использование множества методов типажа `Iterator`
</span>

Обратите внимание, что `zip` возвращает только четыре пары; теоретическая пятая 
пара `(5, None)` никогда не создастся, поскольку `zip` возвращает `None`, когда 
любой из его входных итераторов возвращает `None`.

Все эти вызовы методов возможны, потому что мы реализовали типаж `Iterator`, 
указав, как работает метод `next`, и стандартная библиотека предоставляет 
реализации по умолчанию для других методов, которые вызывают `next`.
