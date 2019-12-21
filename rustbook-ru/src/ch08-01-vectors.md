## Векторы

Первым типом коллекции, который мы разберем будет *вектор* `Vec<T>`. Векторы позволяют сохранять множество данных в одной структуре, сохраняя элементы в памяти один за другим. Векторы могут сохранять данные только одного типа. Они удобны, когда нужно иметь список элементов, такие как текстовые строки в файле или цены в списке корзины покупок.

### Создание нового вектора

Для создания нового вектора используется функция `Vec::new`, как показано в листинге 8-1.

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    print!("{:?} ", v);
}
```

<span class="caption">Listing 8-1: Creating a new, empty vector to hold values
of type <code>i32</code></span>

Обратите внимание, что мы добавили описание (аннотацию) типа данных. Очень важный
момент: пока мы не добавим хотя бы один элемент в вектор, компилятор (Rust) не будет
знать, что за тип данных будет содержаться в этой коллекции. Как мы уже говорили,
вектор может содержать только один тип данных. Это его особенность.

Более удобный способ инициализации вектора - с помощью макроса `vec!` (по умолчанию
тип числовых данных i32):

```rust
let v = vec![1, 2, 3];
```

<span class="caption">Листинг 8-2. Создание нового вектора, содержащего значения</span>

Так как мы создали коллекцию скалярных значений, то компилятор на основе типов
данных самостоятельно установит тип данных вектора.
Интересно, какой тип данных будет у вектора, если сделать такие изменения в коде
инициализации:

### Изменение вектора

Чтобы создать вектор и затем добавить к нему элементы, можно использовать метод `push` показанный в листинге 8-3.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

<span class="caption">Листинг 8-3. Использование метода <code>push</code> для добавления значений в вектор</span>

As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don’t need the `Vec<i32>` annotation.

### Удаление элементов из вектора

Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-4.

```rust
{
    let v = vec![1, 2, 3, 4];

    // использование вектора v

} // <- v уходит из области видимости и здесь память освобождена
```

<span class="caption">Листинг 8-4. Показывает где вектор и его элементы удалены</span>

Когда вектор удаляется, все его содержимое также удаляется, что означает и удаление чисел, которые он содержит. Это может показаться простой концепцией, но все становится немного сложнее, когда вы начинаете вводить ссылки на элементы вектора. Давайте займемся этим далее!

### Чтение данных вектора

Теперь, когда вы знаете, как создавать, обновлять и уничтожать вектор, знание про чтение его содержимого является следующим шагом. Есть два способа ссылаться на значение хранящееся в векторе. В примерах мы аннотировали типы значений возвращаемых из функций для большей ясности.

Listing 8-5 shows both methods of accessing a value in a vector, either with
indexing syntax or the `get` method.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

<span class="caption">Листинг 8-5. Использование индексного синтаксиса или метода <code>get</code> для получения доступа к элементу вектора</span>

Note two details here. First, we use the index value of `2` to get the third
element: vectors are indexed by number, starting at zero. Second, the two ways
to get the third element are by using `&` and `[]`, which gives us a reference,
or by using the `get` method with the index passed as an argument, which gives
us an `Option<&T>`.

У Rust есть два способа сослаться на элемент, так что вы можете выбрать, как программа ведет себя при попытке использовать значение индекса, для которого в векторе нет элемента. В качестве примера, давайте посмотрим что будет делать программа если у нее есть вектор, который содержит пять элементов, а мы пытаемся получить доступ к элементу с индексом 100, как показано в листинге 8-6.

```rust,should_panic,panics
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

<span class="caption">Листинг 8-6. Попытка доступа к элементу вектора по индексу 100, содержащему всего пять элементов</span>

Когда мы запустим этот код, первый метод `[]` вызовет в программе панику, потому что он ссылается на несуществующий элемент. Этот метод лучше всего использовать, когда вы хотите, чтобы ваша программа аварийно завершала работу при попытке доступа к элементу за пределами вектора.

When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector happens occasionally under normal circumstances.
Your code will then have logic to handle having either `Some(&element)` or
`None`, as discussed in Chapter 6. For example, the index could be coming from
a person entering a number. If they accidentally enter a number that’s too
large and the program gets a `None` value, you could tell the user how many
items are in the current vector and give them another chance to enter a valid
value. That would be more user-friendly than crashing the program due to a typo!

Когда в программе есть действительная ссылка, средство проверки заимствований обеспечивает выполнение правил владения и заимствования (описанные в главе 4), чтобы обеспечить этой ссылке и любым другим ссылкам на содержимое вектора, остаться действительными. Напомним, правило гласит, что вы не можете иметь изменяемые и неизменные ссылки в одной области видимости. Это правило применяется в листинге 8-7, где мы имеем неизменяемую ссылку на первый элемент вектора и пытаемся добавить некоторый элемент в его конец. Код не будет работать.

```rust,ignore,does_not_compile
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

<span class="caption">Листинг 8-7. Попытка добавить некоторый элемент в вектор, удерживая неизменяемую ссылку на элемент</span>

Компиляция этого кода приведет к ошибке:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```

Код листинга 8-7 может выглядеть так, как будто он должен бы работать: почему ссылка на первый элемент должна волноваться о том, что меняется в конце вектора? Это ошибка связана с тем, как работают векторы: добавление нового элемента в конец вектора может потребовать выделения нового блока памяти и копирования старых элементов в новое пространство, если будет недостаточно места для размещения всех  элементов рядом друг с другом там, где вектор находится в данное время. В этом случае ссылка на первый элемент будет указывать на освобожденную память. Правила заимствования предотвращают возможность программе оказаться в такой ситуации.

> Примечание. Дополнительные сведения о реализации типа `Vec<T>` см. в разделе «The 
> Rustonomicon » по адресу https://doc.rust-lang.org/stable/nomicon/vec.html.

### Iterating over the Values in a Vector

Если нужно получить доступ к каждому элементу вектора по очереди, можно перебирать все элементы, а не использовать индексы для доступа по одному в один момент времени. В листинге 8-8 показано, как использовать цикл `for` для получения неизменяемых ссылок на каждый элемент вектора со значениями типа `i32` и распечатывания их.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

<span class="caption">Листинг 8-8. Печать каждого элемента в векторе, перебирая элементы с помощью цикла <code>for</code></span>

Мы также можем перебирать изменяемые ссылки на каждый элемент изменяемого вектора для того, чтобы внести изменения во все элементы. Цикл `for` в листинге 8-9 добавит число `50` к каждому элементу.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

<span class="caption">Листинг 8-9. Итерация по изменяемым ссылкам на элементы вектора</span>

To change the value that the mutable reference refers to, we have to use the
dereference operator (`*`) to get to the value in `i` before we can use the
`+=` operator. We’ll talk more about the dereference operator in the
[“Following the Pointer to the Value with the Dereference Operator”](ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator)
section of Chapter 15.

### Использование перечисления для хранения множества разных типов

В начале этой главы мы говорили, что векторы могут хранить только значения одинакового типа. Это может быть неудобно; безусловно есть случаи, когда необходим список из элементов разного типа. К счастью, варианты перечисления определены в самом типе перечисления, поэтому когда нам нужно хранить в векторе элементы различного типа, можно определить и использовать перечисление!

Например, мы хотим получить значения из строки в электронной таблице где некоторые столбцы строки содержат целые числа, некоторые числа с плавающей точкой или строковые значение. Можно определить перечисление, варианты которого будут содержать разные типы значений и тогда все варианты перечисления будут считаться одними и тем же типом: самим перечислением. Затем можно создать вектор, который содержит это перечисление и таким образом в конечном счете содержит разные типы. Мы продемонстрировали это в листинге 8-10.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

<span class="caption">Листинг 8-10. Определение <code>enum</code> для хранения значений разных типов в векторе</span>

Rust должен знать, какие типы будут в векторе во время компиляции, чтобы знать сколько именно памяти в куче потребуется для хранения каждого элемента. Вторичным преимуществом является то, что мы можем четко указать какие типы разрешены в таком векторе. Если бы Rust позволял вектору содержать любой тип, был бы шанс что один или несколько типов будут вызывать ошибки при выполнении операций над элементами вектора. Используя перечисление вместе с выражением `match` означает, что во время компиляции Rust гарантирует  обработку каждого возможного случая, как обсуждалось в главе 6.

При написании программы, если вы не знаете исчерпывающий набор типов программы во время выполнения для сохранения их в вектор,  то техника использования перечисления не будет работать. Вместо этого вы можете использовать объект типажа, который мы рассмотрим в главе 17.

Now that we’ve discussed some of the most common ways to use vectors, be sure
to review the API documentation for all the many useful methods defined on
`Vec<T>` by the standard library. For example, in addition to `push`, a `pop`
method removes and returns the last element. Let’s move on to the next
collection type: `String`!
