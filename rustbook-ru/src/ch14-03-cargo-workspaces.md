## Рабочие пространства Cargo

В главе 12 мы создали дополнение, который включал в себя двоичный и библиотечный ящики. По мере развития вашего дела может возникнуть случай, когда библиотечный ящик будет становиться все больше, и вы захотите разделить ваш дополнение на несколько библиотечных ящиков. Cargo предоставляет возможность под названием *workspaces*, которая помогает управлять несколькими взаимосвязанными дополнениями, которые разрабатываются в тандеме.

### Создание рабочего пространства

*Workspace* - это набор дополнений, которые используют один и тот же *Cargo.lock* и папку для хранения итогов сборки. Давайте создадим дело с использованием *workspace* - мы будем использовать обыкновенный рукопись, чтобы сосредоточиться на устройстве рабочего пространства. Существует несколько способов внутренне выстроить

 рабочую область, но мы покажем только один из них. У нас будет рабочая область, содержащая двоичный файл и две библиотеки. Двоичный файл, который обеспечивает основную возможность, будет зависеть от двух библиотек. Одна библиотека предоставит функцию `add_one`, а вторая - `add_two`. Эти три ящика будут частью одного *workspace*. Начнём с создания папки для рабочего окружения:

```console
$ mkdir add
$ cd add
```

Далее в папке *add* мы создадим файл *Cargo.toml*, который будет определять настройку всего рабочего окружения. В этом файле не будет разделы `[package]`. Вместо этого он будет начинаться с разделы `[workspace]`, которая позволит нам добавить разделы в рабочее пространство, указав путь к дополнению с нашим двоичным ящиком; в данном случае этот путь - *adder*:

<span class="filename">Файл: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```

Затем мы создадим исполняемый ящик `adder`, запустив приказ `cargo new` в папке *add*:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
```

На этом этапе мы можем создать рабочее пространство, запустив  приказ `cargo build`. Файлы в папке *add* должны выглядеть следующим образом:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Рабочая область содержит на верхнем уровне одна папка *target*, в который будут помещены собранные артефакты; дополнение `adder` не имеет собственного папки *target*. Даже если мы запустим `cargo build` из папки *adder*, собранные артефакты все равно окажутся в *add/target*, а не в *add/adder/target*. Cargo так определил папку *target* в рабочем пространстве, потому что ящики в рабочем пространстве должны зависеть друг от друга. Если бы каждый ящик имел свою собственную папку *target*, каждому ящику пришлось бы пересобирать каждый из других ящиков в рабочем пространстве, чтобы поместить артефакты в свою собственную папку *target*. Благодаря совместному использованию единого папки *target* ящики могут избежать ненужной пересборки.

### Добавление второго ящика в рабочее пространство

Далее давайте создадим ещё одного участника дополнения в рабочей области и назовём его `add_one`. Внесите изменения в *Cargo.toml* верхнего уровня так, чтобы указать путь *add_one* в списке `members`:

<span class="filename">Файл: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Затем создайте новый ящик библиотеки с именем `add_one`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
```

Ваш папка *add* должна теперь иметь следующие папки и файлы:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

В файле *add_one/src/lib.rs* добавим функцию `add_one`:

<span class="filename">Файл: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

Теперь мы можем сделать так, чтобы дополнение `adder` с нашим исполняемым файлом зависел от дополнения `add_one`, содержащего нашу библиотеку. Сначала нам нужно добавить в дополнение пути от `add_one` в *adder/Cargo.toml*.

<span class="filename">Файл: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo не исходит из того, что ящики в рабочем пространстве могут зависеть друг от друга, поэтому нам необходимо явно указать отношения зависимости.

Далее, давайте используем функцию `add_one` (из ящика `add_one`) в ящике `adder`. Откройте файл *adder/src/main.rs* и добавьте строку `use` в верхней части, чтобы ввести в область видимости новый библиотечный ящик `add_one`. Затем измените функцию `main` для вызова функции `add_one`, как показано в приложении 14-7.

<span class="filename">Файл: adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<span class="caption">Приложение 14-7: Использование возможностей библиотечного ящика <code>add-one</code> в ящике <code>adder</code></span>

Давайте соберём  рабочее пространство, запустив приказ `cargo build` в папке верхнего уровня *add*!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

Чтобы запустить двоичный ящик из папки *add*, нам нужно указать какой дополнение из рабочей области мы хотим использовать с помощью переменной `-p` и названия дополнения в приказу `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

Запуск рукописи из *adder/src/main.rs*, который зависит от `add_one`.

#### Зависимость от внешних ящиков в рабочем пространстве

Обратите внимание, что рабочая область имеет один единственный файл *Cargo.lock* на верхнем уровне, а не содержит *Cargo.lock* в папке каждого ящика. Это заверяет, что все ящики используют одну и ту же исполнение всех дополнений. Если мы добавим дополнение `rand` в файлы *adder/Cargo.toml* и *add_one/Cargo.toml*, Cargo сведёт их оба к одной исполнения `rand` и запишет её в один файл *Cargo.lock*. Если заставить все ящики в рабочей области использовать одни и те же дополнения, то это будет означать, что ящики всегда будут совместимы друг с другом. Давайте добавим ящик `rand` в раздел `[dependencies]` в файле *add_one/Cargo.toml*, чтобы мы могли использовать ящик `rand` в ящике `add_one`:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Файл: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

Теперь мы можем добавить `use rand;` в файл  *add_one/src/lib.rs* и сделать сборку рабочего пространства, запустив `cargo build` в папке *add*, что загрузит и собирает `rand` ящик:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

Файл *Cargo.lock* верхнего уровня теперь содержит сведения о зависимости `add_one` к ящику `rand`. Тем не менее, не смотря на то что `rand` использован где-то в рабочем пространстве, мы не можем использовать его в других ящиках рабочего пространства, пока не добавим ящик `rand` в отдельные *Cargo.toml* файлы. Например, если мы добавим `use rand;` в файл *adder/src/main.rs* ящика  `adder`, то получим ошибку:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Чтобы исправить это, изменените файл *Cargo.toml* для дополнения `adder` и укажите, что `rand` также является его дополнением. При сборке дополнения `adder` `rand` будет добавлен в список дополнений для `adder` в *Cargo.lock*, но никаких дополнительных повторов `rand` загружено не будет. Cargo позаботился о том, чтобы все ящики во всех дополнениях рабочей области, использующих дополнение `rand`, использовали одну и ту же исполнение, экономя нам место и обеспечивая, что все ящики в рабочей области будут совместимы друг с другом.

#### Добавление проверки в рабочее пространство

В качестве ещё одного улучшения давайте добавим проверка функции `add_one::add_one` в `add_one`:

<span class="filename">Файл: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Теперь запустите `cargo test` в папке верхнего уровня *add*. Запуск `cargo test` в рабочем пространстве, внутренне упорядоченном подобно этому примеру, запустит проверки для всех ящиков в рабочем пространстве:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Первая раздел вывода показывает, что проверка `it_works` в ящике `add_one` прошёл. Следующая раздел показывает, что в ящике `adder` не было обнаружено ни одного проверки, а последняя раздел показывает, что в ящике `add_one` не было найдено ни одного проверки пособия.

Мы также можем запустить проверки для одного определенного ящика в рабочем пространстве из папки верхнего уровня с помощью клейма `-p` и указанием имени ящика для которого мы хотим запустить проверки:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Эти выходные данные показывают, что использование `cargo test` запускает только проверки для ящика `add-one` и не запускает проверки ящика `adder`.

Если вы соберётесь обнародовать ящики из рабочего пространства на [crates.io](https://crates.io/), каждый ящик будет необходимо будет обнародовать отдельно. Подобно `cargo test`, мы можем обнародовать определенный ящик из нашей рабочей области, используя клеймо `-p` и указав имя ящика, который мы хотим обнародовать.

Для дополнительной опытов добавьте ящик `add_two` в данное рабочее пространство подобным способом, как делали с ящик `add_one` !

По мере роста дела рассмотрите возможность использования рабочих областей: легче понять небольшие, отдельные составляющие, чем один большой кусок рукописи. Кроме того, хранение ящиков в рабочем пространстве может облегчить согласование между ящиками, если они часто изменяются одновременно.
