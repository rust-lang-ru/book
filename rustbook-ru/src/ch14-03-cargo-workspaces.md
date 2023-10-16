## Рабочие пространства Cargo

В главе 12 мы создали пакет, который включал в себя бинарный и библиотечный крейты. По мере развития вашего проекта может возникнуть ситуация, когда библиотечный крейт будет становиться все больше, и вы захотите разделить ваш пакет на несколько библиотечных крейтов. Cargo предоставляет функциональность под названием *workspaces*, которая помогает управлять несколькими взаимосвязанными пакетами, которые разрабатываются в тандеме.

### Создание рабочего пространства

*Workspace* - это набор пакетов, которые используют один и тот же *Cargo.lock* и директорию для хранения результатов компиляции. Давайте создадим проект с использованием *workspace* - мы будем использовать тривиальный код, чтобы сосредоточиться на структуре рабочего пространства. Существует несколько способов структурировать рабочую область, но мы покажем только один из них. У нас будет рабочая область, содержащая двоичный файл и две библиотеки. Двоичный файл, который обеспечивает основную функциональность, будет зависеть от двух библиотек. Одна библиотека предоставит функцию `add_one`, а вторая - `add_two`. Эти три крейта будут частью одного *workspace*. Начнём с создания каталога для рабочего окружения:

```console
$ mkdir add
$ cd add
```

Далее в каталоге *add* мы создадим файл *Cargo.toml*, который будет определять конфигурацию всего рабочего окружения. В этом файле не будет секции `[package]`. Вместо этого он будет начинаться с секции `[workspace]`, которая позволит нам добавить модули в рабочее пространство, указав путь к пакету с нашим бинарным крейтом; в данном случае этот путь - *adder*:

<span class="filename">Файл: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```

Затем мы создадим исполняемый крейт `adder`, запустив команду `cargo new` в каталоге *add*:

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

На этом этапе мы можем создать рабочее пространство, запустив  команду `cargo build`. Файлы в каталоге *add* должны выглядеть следующим образом:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Рабочая область содержит на верхнем уровне один каталог *target*, в который будут помещены скомпилированные артефакты; пакет `adder` не имеет собственного каталога *target*. Даже если мы запустим `cargo build` из каталога *adder*, скомпилированные артефакты все равно окажутся в *add/target*, а не в *add/adder/target*. Cargo так определил директорию *target* в рабочем пространстве, потому что крейты в рабочем пространстве должны зависеть друг от друга. Если бы каждый крейт имел свой собственный каталог *target*, каждому крейту пришлось бы перекомпилировать каждый из других крейтов в рабочем пространстве, чтобы поместить артефакты в свой собственный каталог *target*. Благодаря совместному использованию единого каталога *target* крейты могут избежать ненужной перекомпиляции.

### Добавление второго крейта в рабочее пространство

Далее давайте создадим ещё одного участника пакета в рабочей области и назовём его `add_one`. Внесите изменения в *Cargo.toml* верхнего уровня так, чтобы указать путь *add_one* в списке `members`:

<span class="filename">Файл: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Затем сгенерируйте новый крейт библиотеки с именем `add_one`:

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

Ваш каталог *add* должен теперь иметь следующие каталоги и файлы:

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

Теперь мы можем сделать так, чтобы пакет `adder` с нашим исполняемым файлом зависел от пакета `add_one`, содержащего нашу библиотеку. Сначала нам нужно добавить зависимость пути от `add_one` в *adder/Cargo.toml*.

<span class="filename">Файл: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo не исходит из того, что крейты в рабочем пространстве могут зависеть друг от друга, поэтому нам необходимо явно указать отношения зависимости.

Далее, давайте используем функцию `add_one` (из крейта `add_one`) в крейте `adder`. Откройте файл *adder/src/main.rs* и добавьте строку `use` в верхней части, чтобы ввести в область видимости новый библиотечный крейт `add_one`. Затем измените функцию `main` для вызова функции `add_one`, как показано в листинге 14-7.

<span class="filename">Файл: adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<span class="caption">Листинг 14-7: Использование функционала библиотечного крейта <code>add-one</code> в крейте <code>adder</code></span>

Давайте соберём  рабочее пространство, запустив команду `cargo build` в каталоге верхнего уровня *add*!

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

Чтобы запустить бинарный крейт из каталога *add*, нам нужно указать какой пакет из рабочей области мы хотим использовать с помощью аргумента `-p` и названия пакета в команде `cargo run`:

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

Запуск кода из *adder/src/main.rs*, который зависит от `add_one`.

#### Зависимость от внешних крейтов в рабочем пространстве

Обратите внимание, что рабочая область имеет один единственный файл *Cargo.lock* на верхнем уровне, а не содержит *Cargo.lock* в каталоге каждого крейта. Это гарантирует, что все крейты используют одну и ту же версию всех зависимостей. Если мы добавим пакет `rand` в файлы *adder/Cargo.toml* и *add_one/Cargo.toml*, Cargo сведёт их оба к одной версии `rand` и запишет её в один *Cargo.lock*. Если заставить все крейты в рабочей области использовать одни и те же зависимости, то это будет означать, что крейты всегда будут совместимы друг с другом. Давайте добавим крейт `rand` в раздел `[dependencies]` в файле *add_one/Cargo.toml*, чтобы мы могли использовать крейт `rand` в крейте `add_one`:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Файл: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

Теперь мы можем добавить `use rand;` в файл  *add_one/src/lib.rs* и сделать сборку рабочего пространства, запустив `cargo build` в каталоге *add*, что загрузит и скомпилирует `rand` крейт:

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

Файл *Cargo.lock* верхнего уровня теперь содержит информацию о зависимости `add_one` к крейту `rand`. Тем не менее, не смотря на то что `rand` использован где-то в рабочем пространстве, мы не можем использовать его в других крейтах рабочего пространства, пока не добавим крейт `rand` в отдельные *Cargo.toml* файлы. Например, если мы добавим `use rand;` в файл *adder/src/main.rs* крейта  `adder`, то получим ошибку:

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

Чтобы исправить это, отредактируйте файл *Cargo.toml* для пакета `adder` и укажите, что `rand` также является его зависимостью. При сборке пакета `adder` `rand` будет добавлен в список зависимостей для `adder` в *Cargo.lock*, но никаких дополнительных копий `rand` загружено не будет. Cargo позаботился о том, чтобы все крейты во всех пакетах рабочей области, использующих пакет `rand`, использовали одну и ту же версию, экономя нам место и гарантируя, что все крейты в рабочей области будут совместимы друг с другом.

#### Добавление теста в рабочее пространство

В качестве ещё одного улучшения давайте добавим тест функции `add_one::add_one` в `add_one`:

<span class="filename">Файл: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Теперь запустите `cargo test` в каталоге верхнего уровня *add*. Запуск `cargo test` в рабочем пространстве, структурированном подобно этому, запустит тесты для всех крейтов в рабочем пространстве:

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

Первая секция вывода показывает, что тест `it_works` в крейте `add_one` прошёл. Следующая секция показывает, что в крейте `adder` не было обнаружено ни одного теста, а последняя секция показывает, что в крейте `add_one` не было найдено ни одного теста документации.

Мы также можем запустить тесты для одного конкретного крейта в рабочем пространстве из каталог верхнего уровня с помощью флага `-p` и указанием имени крейта для которого мы хотим запустить тесты:

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

Эти выходные данные показывают, что выполнение `cargo test` запускает только тесты для крейта `add-one` и не запускает тесты крейта `adder`.

Если вы соберётесь опубликовать крейты из рабочего пространства на [crates.io](https://crates.io/), каждый крейт будет необходимо будет опубликовать отдельно. Подобно `cargo test`, мы можем опубликовать конкретный крейт из нашей рабочей области, используя флаг `-p` и указав имя крейта, который мы хотим опубликовать.

Для дополнительной практики добавьте крейт `add_two` в данное рабочее пространство аналогичным способом, как делали с крейт `add_one` !

По мере роста проекта рассмотрите возможность использования рабочих областей: легче понять небольшие, отдельные компоненты, чем один большой кусок кода. Кроме того, хранение крейтов в рабочем пространстве может облегчить координацию между крейтами, если они часто изменяются параллельно.
