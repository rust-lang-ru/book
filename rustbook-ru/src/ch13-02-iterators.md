## Обработка группы элементов с помощью итераторов

Шаблон итератора позволяет выполнять некоторые задачи над последовательностью элементов. Итератор отвечает за логику итерации по каждому элементу и определяет, когда последовательность завершилась. Когда вы используете итераторы, вам не нужно переопределять эту логику самостоятельно.

In Rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up. For example, the code in Listing 13-10 creates an iterator over the items in the vector `v1` by calling the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything useful.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

<span class="caption">Listing 13-10: Creating an iterator</span>

The iterator is stored in the `v1_iter` variable. Once we’ve created an iterator, we can use it in a variety of ways. In Listing 3-5 in Chapter 3, we iterated over an array using a `for` loop to execute some code on each of its items. Under the hood this implicitly created and then consumed an iterator, but we glossed over how exactly that works until now.

In the example in Listing 13-11, we separate the creation of the iterator from the use of the iterator in the `for` loop. When the `for` loop is called using the iterator in `v1_iter`, each element in the iterator is used in one iteration of the loop, which prints out each value.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<span class="caption">Listing 13-11: Using an iterator in a <code>for</code> loop</span>

В языках, которые не имеют итераторов в стандартной библиотеке, вы, вероятно, написали бы эту же функцию следующим образом: взять переменную со значением 0, использовать её для индексации вектора, чтобы получить значение, и увеличивать её значение в цикле, пока не будет достигнуто общее количество элементов в векторе.

Итераторы делают все эти шаги за вас, сокращая повторяющийся код, который вы потенциально могли бы испортить. Итераторы дают вам больше гибкости для использования одной и той же логики с различными типами последовательностей, а не только со структурами данных, которые можно индексировать, типа векторов. Давайте посмотрим как итераторы это делают.

### Типаж `Iterator` и метод `next`

Все итераторы реализуют типаж `Iterator`, который определён в стандартной библиотеке. Его определение выглядит так:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Обратите внимание данное объявление использует новый синтаксис: `type Item` и `Self::Item`, которые определяют *ассоциированный тип* (associated type) с этим типажом. Мы подробнее поговорим о ассоциированных типах в главе 19. Сейчас вам нужно знать, что этот код требует от реализаций типажа `Iterator` определить требуемый им тип `Item` и данный тип `Item` используется в методе `next`. Другими словами, тип `Item` будет являться типом элемента, который возвращает итератор.

Типаж `Iterator` требует, чтобы разработчики определяли только один метод: метод `next`, который возвращает один элемент итератора за раз обёрнутый в вариант `Some` и когда итерация завершена, возвращает `None`.

We can call the `next` method on iterators directly; Listing 13-12 demonstrates what values are returned from repeated calls to `next` on the iterator created from the vector.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

<span class="caption">Listing 13-12: Calling the <code>next</code> method on an iterator</span>

Обратите внимание, что нам нужно сделать переменную `v1_iter` изменяемой: вызов метода `next` итератора изменяет внутреннее состояние итератора, которое итератор использует для отслеживания того, где он находится в последовательности. Другими словами, этот код *потребляет* (consumes) или использует итератор. Каждый вызов `next` потребляет элемент из итератора. Нам не нужно было делать изменяемой `v1_iter` при использовании цикла `for`, потому что цикл забрал во владение `v1_iter` и сделал её изменяемой неявно для нас.

Заметьте также, что значения, которые мы получаем при вызовах `next` являются неизменяемыми ссылками на значения в векторе. Метод `iter` создаёт итератор по неизменяемым ссылкам. Если мы хотим создать итератор, который становится владельцем `v1` и возвращает принадлежащие ему значения, мы можем вызвать `into_iter` вместо `iter`. Точно так же, если мы хотим перебирать изменяемые ссылки, мы можем вызвать `iter_mut` вместо `iter`.

### Методы, которые потребляют итератор

У типажа `Iterator` есть несколько методов, реализация которых по умолчанию предоставляется стандартной библиотекой; вы можете узнать об этих методах, просмотрев документацию API стандартной библиотеки для `Iterator`. Некоторые из этих методов вызывают `next` в своём определении, поэтому вам необходимо реализовать метод `next` при реализации типажа `Iterator`.

Methods that call `next` are called *consuming adaptors*, because calling them uses up the iterator. One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. Listing 13-13 has a test illustrating a use of the `sum` method:

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

<span class="caption">Listing 13-13: Calling the <code>sum</code> method to get the total of all items in the iterator</span>

Мы не можем использовать `v1_iter` после вызова метода `sum`, потому что `sum` забирает по владение итератор у которого вызван метод.

### Методы, которые создают другие итераторы

*Iterator adaptors* are methods defined on the `Iterator` trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.

Listing 13-17 shows an example of calling the iterator adaptor method `map`, which takes a closure to call on each item as the items are iterated through. The `map` method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1:

<span class="filename">Файл: src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<span class="caption">Listing 13-14: Calling the iterator adaptor <code>map</code> to create a new iterator</span>

However, this code produces a warning:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

The code in Listing 13-14 doesn’t do anything; the closure we’ve specified never gets called. The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.

To fix this warning and consume the iterator, we’ll use the `collect` method, which we used in Chapter 12 with `env::args` in Listing 12-1. This method consumes the iterator and collects the resulting values into a collection data type.

In Listing 13-15, we collect the results of iterating over the iterator that’s returned from the call to `map` into a vector. This vector will end up containing each item from the original vector incremented by 1.

<span class="filename">Файл: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

<span class="caption">Listing 13-15: Calling the <code>map</code> method to create a new iterator and then calling the <code>collect</code> method to consume the new iterator and create a vector</span>

Поскольку `map` принимает замыкание, мы можем указать любую операцию, которую хотим выполнить с каждым элементом. Это отличный пример того, как замыкания позволяют настраивать какое-то поведение при повторном использовании итерационного поведения, предоставляемого типажом `Iterator` .

You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

### Использование замыканий, которые захватывают переменные окружения

Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.

For this example, we’ll use the `filter` method that takes a closure. The closure gets an item from the iterator and returns a `bool`. If the closure returns `true`, the value will be included in the iteration produced by `filter`. If the closure returns `false`, the value won’t be included.

In Listing 13-16, we use `filter` with a closure that captures the `shoe_size` variable from its environment to iterate over a collection of `Shoe` struct instances. It will return only shoes that are the specified size.

<span class="filename">Файл: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

<span class="caption">Listing 13-16: Using the <code>filter</code> method with a closure that captures <code>shoe_size</code></span>

Функция `shoes_in_size` принимает в качестве параметров вектор с экземплярами обуви и размер обуви, а возвращает вектор, содержащий только обувь указанного размера.

В теле `shoes_in_my_size` мы вызываем `into_iter` чтобы создать итератор, который становится владельцем вектора. Затем мы вызываем `filter`, чтобы превратить этот итератор в другой, который содержит только элементы, для которых замыкание возвращает `true`.

Замыкание захватывает параметр `shoe_size` из окружения и сравнивает его с размером каждой пары обуви, оставляя только обувь указанного размера. Наконец, вызов `collect` собирает значения, возвращаемые адаптированным итератором, в вектор, возвращаемый функцией.

Тест показывает, что когда мы вызываем `shoes_in_my_size`, мы возвращаем только туфли, размер которых совпадает с указанным нами значением.
