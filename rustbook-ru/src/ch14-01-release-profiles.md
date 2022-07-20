## Настройка сборок с профилями релизов

В Rust *профили выпуска* — это предопределённые и настраиваемые профили с различными конфигурациями, которые позволяют программисту лучше контролировать различные параметры компиляции кода. Каждый профиль настраивается независимо от других.

Cargo имеет два основных профиля: профиль `dev`, используемый Cargo при запуске `cargo build`, и профиль `release`, используемый Cargo при запуске `cargo build --release`. Профиль `dev` определён со значениями по умолчанию для разработки, а профиль `release` имеет значения по умолчанию для сборок в релиз.

Эти имена профилей могут быть знакомы по результатам ваших сборок:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

The `dev` and `release` are these different profiles used by the compiler.

Cargo has default settings for each of the profiles that apply when you haven't explicitly added any `[profile.*]` sections in the project’s *Cargo.toml* file. By adding `[profile.*]` sections for any profile you want to customize, you override any subset of the default settings. For example, here are the default values for the `opt-level` setting for the `dev` and `release` profiles:

<span class="filename">Файл: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Параметр `opt-level` управляет количеством оптимизаций, которые Rust будет применять к вашему коду, в диапазоне от 0 до 3. Использование большего количества оптимизаций увеличивает время компиляции, поэтому если вы находитесь в процессе разработки и часто компилируете свой код, целесообразно использовать меньшее количество оптимизаций, чтобы компиляция происходила быстрее, даже если в результате код будет работать медленнее. Поэтому `opt-level` по умолчанию для `dev` установлен в `0`. Когда вы готовы опубликовать свой код, то лучше потратить больше времени на компиляцию. Вы скомпилируете программу в режиме релиза только один раз, но выполняться она будет многократно, так что использование режима релиза позволяет увеличить скорость выполнения кода за счёт времени компиляции. Вот почему по умолчанию `opt-level` для профиля `release` равен `3`.

You can override a default setting by adding a different value for it in *Cargo.toml*. For example, if we want to use optimization level 1 in the development profile, we can add these two lines to our project’s *Cargo.toml* file:

<span class="filename">Файл: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Этот код переопределяет настройку по умолчанию `0`. Теперь, когда мы запустим `cargo build`, Cargo будет использовать значения по умолчанию для профиля `dev` плюс нашу настройку для `opt-level`. Поскольку мы установили для `opt-level` значение `1`, Cargo будет применять больше оптимизаций, чем было задано по умолчанию, но не так много, как при сборке релиза.

Полный список параметров конфигурации и значений по умолчанию для каждого профиля вы можете найти в [документации Cargo](https://doc.rust-lang.org/cargo/reference/profiles.html).
