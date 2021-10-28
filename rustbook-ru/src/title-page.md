# Язык программирования Rust

*от авторов Steve Klabnik, Carol Nichols и участников сообщества Rust*

This version of the text assumes you’re using Rust 1.55 or later with `edition="2018"` in *Cargo.toml* of all projects to use Rust 2018 Edition idioms. See the [“Installation” section of Chapter 1](ch01-01-installation.html)<!-- ignore --> to install or update Rust, and see the new [Appendix E](appendix-05-editions.html)<!-- ignore --> for information on editions.

Язык Rust редакции 2018 года включает в себя ряд улучшений, которые делают Rust более эргономичным и лёгким в освоении. Эта версия книги содержит ряд изменений, отражающих эти улучшения:

- Chapter 7, “Managing Growing Projects with Packages, Crates, and Modules,” has been mostly rewritten. The module system and the way paths work in the 2018 Edition were made more consistent.
- Глава 10 обзавелась новыми разделами - "Типажи как параметры" и "Возврат типов, реализующих типаж", которые разъясняют новый синтаксис `impl Trait`.
- В главе 11 добавлен раздел "Использование `Result<T, E>` в тестах", который показывает как писать тесты, использующие оператор `?`.
- The “Advanced Lifetimes” section in Chapter 19 was removed because compiler improvements have made the constructs in that section even rarer.
- The previous Appendix D, “Macros,” has been expanded to include procedural macros and was moved to the “Macros” section in Chapter 19.
- Appendix A, “Keywords,” also explains the new raw identifiers feature that enables code written in the 2015 Edition and the 2018 Edition to interoperate.
- Appendix D is now titled “Useful Development Tools” and covers recently released tools that help you write Rust code.
- We fixed a number of small errors and imprecise wording throughout the book. Thank you to the readers who reported them!

Обратите внимание, что любой код из более ранних версий *книги*, продолжит компилироваться без указания `edition="2018"` в *Cargo.toml* проекта, даже если вы обновите используемую версию компилятора Rust. Это гарантия обратной совместимости Rust!

HTML-версия книги доступна онлайн по адресам [https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)<sub>(англ.)</sub> и [https://doc.rust-lang.ru/book](https://doc.rust-lang.ru/book)<sub>(рус.)</sub> и оффлайн, при установке Rust с помощью `rustup`: просто запустите `rustup docs --book` чтобы открыть её.

Английский вариант книги доступен [в печатном виде и в ebook формате от No Starch Press](https://nostarch.com/rust).
