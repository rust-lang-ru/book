## Хранение ключей со связанными значениями в хэш-картах

Последняя коллекция, которую мы рассмотрим в нашей книге будет *hash map*.
`HashMap<K, V>` сохраняет ключи типа `K` и значения типа `V`. Данная структура организует и хранит данные
с помощью *функции хэширования*. Во множестве библиотек языков программирования
реализована данная структура и функционал. Все они, что неудивительно, имеют разные
наименования: hash, map, object, hash table, или ассоциированный массив.

Hash maps are useful when you want to look up data not by using an index, as
you can with vectors, but by using a key that can be of any type. For example,
in a game, you could keep track of each team’s score in a hash map in which
each key is a team’s name and the values are each team’s score. Given a team
name, you can retrieve its score.

We’ll go over the basic API of hash maps in this section, but many more goodies
are hiding in the functions defined on `HashMap<K, V>` by the standard library.
As always, check the standard library documentation for more information.

### Создание новой хэш-карты

You can create an empty hash map with `new` and add elements with `insert`. In
Listing 8-20, we’re keeping track of the scores of two teams whose names are
Blue and Yellow. The Blue team starts with 10 points, and the Yellow team
starts with 50.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

<span class="caption">Listing 8-20: Creating a new hash map and inserting some
keys and values</span>

Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values
must have the same type.

Ещё один способ создания `HashMap`, использование метода вектора кортежей `collect`,
где каждый кортеж содержит ключ и его значение. Этот метод может объединять любые
типы данных, даже `HashMap`. Например, если у нас есть список команд и значения
счёта этих команд в двух различных векторах, мы можем использовать метод `zip`,
чтобы создать вектор кортежей, где элементы с одинаковыми индексами образуют пары
"ключ-значение". Далее, мы можем использовать метод `collect` для создания `HashMap`:

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

<span class="caption">Listing 8-21: Creating a hash map from a list of teams
and a list of scores</span>

The type annotation `HashMap<_, _>` is needed here because it’s possible to
`collect` into many different data structures and Rust doesn’t know which you
want unless you specify. For the parameters for the key and value types,
however, we use underscores, and Rust can infer the types that the hash map
contains based on the types of the data in the vectors.

### Хеш-карты и владение

For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>

We aren’t able to use the variables `field_name` and `field_value` after
they’ve been moved into the hash map with the call to `insert`.

If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We’ll talk more about these issues in
the [“Validating References with
Lifetimes”](ch10-03-lifetime-syntax.html#validating-references-with-lifetimes)<comment> section in
Chapter 10.</comment>

### Доступ к данным

Для получения данных из `HashMap` используется метод `get`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

<span class="caption">Listing 8-23: Accessing the score for the Blue team
stored in the hash map</span>

Here, `score` will have the value that’s associated with the Blue team, and the
result will be `Some(&10)`. The result is wrapped in `Some` because `get`
returns an `Option<&V>`; if there’s no value for that key in the hash map,
`get` will return `None`. The program will need to handle the `Option` in one
of the ways that we covered in Chapter 6.

We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

Этот код будет печатать каждую пару в произвольном порядке:

```text
Yellow: 50
Blue: 10
```

### Обновление данных

Although the number of keys and values is growable, each key can only have one
value associated with it at a time. When you want to change the data in a hash
map, you have to decide how to handle the case when a key already has a value
assigned. You could replace the old value with the new value, completely
disregarding the old value. You could keep the old value and ignore the new
value, only adding the new value if the key *doesn’t* already have a value. Or
you could combine the old value and the new value. Let’s look at how to do each
of these!

#### Перезаписывание старых данных

If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-24 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

<span class="caption">Listing 8-24: Replacing a value stored with a particular
key</span>

Будет напечатано `{"Blue": 25}`. Первое значение 10 будет перезаписано.

#### Только вставка значения, если ключ не имеет значения

It’s common to check whether a particular key has a value and, if it doesn’t,
insert a value for it. Hash maps have a special API for this called `entry`
that takes the key you want to check as a parameter. The return value of the
`entry` method is an enum called `Entry` that represents a value that might or
might not exist. Let’s say we want to check whether the key for the Yellow team
has a value associated with it. If it doesn’t, we want to insert the value 50,
and the same for the Blue team. Using the `entry` API, the code looks like
Listing 8-25.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

<span class="caption">Listing 8-25: Using the <code>entry</code> method to only insert if
the key does not already have a value</span>

The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-25 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.

#### Обновление значения основанное на предыдущих данных

Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-26 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

<span class="caption">Listing 8-26: Counting occurrences of words using a hash
map that stores words and counts</span>

Будет напечатано `{"world": 2, "hello": 1, "wonderful": 1}`. Метод `or_insert`
возвращает изменяемую ссылку (`&mut V`) по ключу. Мы сохраняем изменяемую ссылку
в переменной `count`. Для того, чтобы присвоить переменной значение, необходимо
произвести разименование с помощью звёздочки (`*`). Изменяемая ссылка удаляется
сразу же после выхода из области видимости цикла `for`. Все эти изменения безопасны
и согласуются с правилами заимствования.

### Функция хэширования

По умолчанию `HashMap` использует криптографическую защитную функцию, которая может
противостоять DOS-атакам. В этой реализации используется не самый быстрый алгоритм
хэширования, но достаточно защищенный. Если после профилирования вашего кода окажется,
что хэш функция очень медленная, вы можете её заменить на другую подобную функцию
(*hasher*). Эта функция реализует поведение `BuildHasher`. Подробнее о поведении
вы узнаете в главе 10. Вам совсем не обязательно реализовывать свою собственную функцию
хэширования. crates.io имеет достаточное количество библиотек для этих целей.

## Summary

Векторы, строки и хэш-карты помогают вам, когда необходимо сохранять, получать доступ
и модифицировать данные. Для закрепления рассмотренного материала, пожалуйста,
выполните следующие учебные задания:

- Есть список целых чисел. Создайте функцию, входной параметр, которой вектор и возвращает: среднее; медиану (значение центрального элемента списка); значение, которое есть в списке набольшее количество раз.
- Сконвертируюйте строку в Pig Latin, где первая согласная каждого слова перемещается в конец и добавлением окончания “ay”. Пример, “first” - “irst-fay”. Если слово начинается на гласную, добавляет в конец слова суффикс “hay”. Пример,   “apple” - “apple-hay”.
- Using a hash map and vectors, create a text interface to allow a user to addemployee names to a department in a company. For example, “Add Sally toEngineering” or “Add Amir to Sales.” Then let the user retrieve a list of allpeople in a department or all people in the company by department, sortedalphabetically.

Документация к стандартной библиотеке достаточно подробна и будет вам помогать в
решении поставленных задач.

We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!
