## Рабочие пространства Cargo

В главе 12 мы создали пакет, который включал бинарный крейт и библиотечный
крейт. По мере развития проекта вы можете обнаружить, что библиотечный крейт
продолжает расти, и захотите дальше разделить пакет на несколько библиотечных
крейтов. Cargo предлагает возможность под названием _рабочие пространства_,
которая помогает управлять несколькими связанными пакетами, разрабатываемыми
совместно.

### Создание рабочего пространства

_Рабочее пространство_ -- это набор пакетов, которые совместно используют один
и тот же _Cargo.lock_ и каталог вывода. Создадим проект с рабочим
пространством: мы будем использовать простой код, чтобы сосредоточиться на
структуре рабочего пространства. Есть несколько способов структурировать
рабочее пространство, поэтому мы покажем один распространенный способ. У нас
будет рабочее пространство с одним бинарным крейтом и двумя библиотеками.
Бинарный крейт, который будет предоставлять основную функциональность, будет
зависеть от двух библиотек. Одна библиотека будет предоставлять функцию
`add_one`, а другая -- функцию `add_two`. Эти три крейта будут частью одного
рабочего пространства. Начнем с создания нового каталога для рабочего
пространства:

```console
$ mkdir add
$ cd add
```

Далее в каталоге _add_ мы создаем файл _Cargo.toml_, который будет настраивать
все рабочее пространство. В этом файле не будет раздела `[package]`. Вместо
этого он начнется с раздела `[workspace]`, который позволит нам добавлять
участников в рабочее пространство. Мы также явно используем самую новую и
лучшую версию алгоритма разрешения зависимостей Cargo в нашем рабочем пространстве, установив
значение `resolver` в `"3"`:

<span class="filename">Имя файла: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

Далее мы создадим бинарный крейт `adder`, выполнив `cargo new` внутри каталога
_add_:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

Запуск `cargo new` внутри рабочего пространства также автоматически добавляет
только что созданный пакет в ключ `members` в определении `[workspace]` в
_Cargo.toml_ рабочего пространства, вот так:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

На этом этапе мы можем собрать рабочее пространство, выполнив `cargo build`.
Файлы в вашем каталоге _add_ должны выглядеть так:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

У рабочего пространства есть один каталог _target_ на верхнем уровне, в который
помещаются скомпилированные артефакты; у пакета `adder` нет собственного
каталога _target_. Даже если бы мы выполнили `cargo build` изнутри каталога
_adder_, скомпилированные артефакты все равно оказались бы в _add/target_, а
не в _add/adder/target_. Cargo так структурирует каталог _target_ в рабочем
пространстве, потому что крейты в рабочем пространстве предназначены для
зависимости друг от друга. Если бы у каждого крейта был собственный каталог
_target_, каждому крейту пришлось бы перекомпилировать каждый из других
крейтов в рабочем пространстве, чтобы поместить артефакты в свой собственный
каталог _target_. Совместно используя один каталог _target_, крейты могут
избежать ненужной пересборки.

### Создание второго пакета в рабочем пространстве

Далее создадим еще один пакет-участник в рабочем пространстве и назовем его
`add_one`. Сгенерируйте новый библиотечный крейт с именем `add_one`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

Теперь _Cargo.toml_ верхнего уровня будет включать путь _add_one_ в список
`members`:

<span class="filename">Имя файла: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Теперь в вашем каталоге _add_ должны быть такие каталоги и файлы:

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

В файл _add_one/src/lib.rs_ добавим функцию `add_one`:

<span class="filename">Имя файла: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

Теперь пакет `adder` с нашим бинарным крейтом может зависеть от пакета
`add_one`, содержащего нашу библиотеку. Сначала нужно добавить зависимость по
пути от `add_one` в _adder/Cargo.toml_.

<span class="filename">Имя файла: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo не предполагает, что крейты в рабочем пространстве будут зависеть друг
от друга, поэтому нам нужно явно указать отношения зависимостей.

Далее используем функцию `add_one` (из крейта `add_one`) в крейте `adder`.
Откройте файл _adder/src/main.rs_ и измените функцию `main`, чтобы она
вызывала функцию `add_one`, как в листинге 14-7.

<Listing number="14-7" file-name="adder/src/main.rs" caption="Использование библиотечного крейта `add_one` из крейта `adder`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

Соберем рабочее пространство, выполнив `cargo build` в каталоге _add_ верхнего
уровня!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

Чтобы запустить бинарный крейт из каталога _add_, можно указать, какой пакет в
рабочем пространстве мы хотим запустить, используя аргумент `-p` и имя пакета
с `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

Это запускает код из _adder/src/main.rs_, который зависит от крейта `add_one`.

<!-- Old headings. Do not remove or links may break. -->

<a id="depending-on-an-external-package-in-a-workspace"></a>

### Зависимость от внешнего пакета

Обратите внимание, что у рабочего пространства есть только один файл
_Cargo.lock_ на верхнем уровне, а не отдельный _Cargo.lock_ в каталоге каждого
крейта. Это гарантирует, что все крейты используют одну и ту же версию всех
зависимостей. Если мы добавим пакет `rand` в файлы _adder/Cargo.toml_ и
_add_one/Cargo.toml_, Cargo сведет обе эти зависимости к одной версии `rand`
и запишет ее в один _Cargo.lock_. Если все крейты в рабочем пространстве
используют одни и те же зависимости, крейты всегда будут совместимы друг с
другом. Добавим крейт `rand` в раздел `[dependencies]` файла
_add_one/Cargo.toml_, чтобы использовать крейт `rand` в крейте `add_one`:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Имя файла: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

Теперь мы можем добавить `use rand;` в файл _add_one/src/lib.rs_, и сборка
всего рабочего пространства командой `cargo build` в каталоге _add_ загрузит и
скомпилирует крейт `rand`. Мы получим одно предупреждение, потому что не
обращаемся к `rand`, который ввели в область видимости:

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

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

Теперь _Cargo.lock_ верхнего уровня содержит информацию о зависимости `add_one`
от `rand`. Однако, хотя `rand` используется где-то в рабочем пространстве, мы
не можем использовать его в других крейтах рабочего пространства, если также не
добавим `rand` в их файлы _Cargo.toml_. Например, если мы добавим `use rand;`
в файл _adder/src/main.rs_ для пакета `adder`, получим ошибку:

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

Чтобы исправить это, отредактируйте файл _Cargo.toml_ для пакета `adder` и
укажите, что `rand` также является его зависимостью. Сборка пакета `adder`
добавит `rand` в список зависимостей `adder` в _Cargo.lock_, но дополнительные
копии `rand` загружены не будут. Cargo проследит, чтобы каждый крейт в каждом
пакете рабочего пространства, использующий пакет `rand`, использовал одну и ту
же версию, пока они указывают совместимые версии `rand`; это экономит место и
гарантирует, что крейты рабочего пространства будут совместимы друг с другом.

Если крейты в рабочем пространстве указывают несовместимые версии одной и той
же зависимости, Cargo разрешит каждую из них, но все равно постарается
разрешить как можно меньше версий.

### Добавление теста в рабочее пространство

В качестве еще одного улучшения добавим тест функции `add_one::add_one` внутри
крейта `add_one`:

<span class="filename">Имя файла: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Теперь выполните `cargo test` в каталоге _add_ верхнего уровня. Запуск
`cargo test` в рабочем пространстве, структурированном таким образом, выполнит тесты
для всех крейтов в рабочем пространстве:

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Первый раздел вывода показывает, что тест `it_works` в крейте `add_one`
прошел. Следующий раздел показывает, что в крейте `adder` найдено ноль тестов,
а последний раздел показывает, что в крейте `add_one` найдено ноль
документационных тестов.

Мы также можем запускать тесты для одного конкретного крейта в рабочем
пространстве из каталога верхнего уровня, используя флаг `-p` и указывая имя
крейта, который хотим тестировать:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Этот вывод показывает, что `cargo test` запустил только тесты для крейта
`add_one` и не запускал тесты крейта `adder`.

Если вы публикуете крейты из рабочего пространства на
[crates.io](https://crates.io/)<!-- ignore -->, каждый крейт в рабочем
пространстве нужно публиковать отдельно. Как и с `cargo test`, мы можем
опубликовать конкретный крейт в нашем рабочем пространстве, используя флаг
`-p` и указывая имя крейта, который хотим опубликовать.

Для дополнительной практики добавьте крейт `add_two` в это рабочее пространство
так же, как крейт `add_one`!

По мере роста проекта подумайте об использовании рабочего пространства: оно
позволяет работать с более маленькими и понятными компонентами, чем один
большой комок кода. Кроме того, хранение крейтов в рабочем пространстве может
упростить координацию между крейтами, если они часто изменяются одновременно.
