# Написание автоматических тестов

В своём эссе 1972 года “The Humble Programmer,” Edsger W. Dijkstra сказал, что «Тестирование программы может быть очень эффективным способом показать наличие ошибок, но это безнадёжно неадекватно для показа их отсутствия». Это не значит, что мы не должны пытаться тестировать столько, сколько мы можем!

Правильность в наших программах - это степень, в которой наш код выполняет то для чего он предназначен. Rust разработан с высокой степенью заботы о правильности программ, но правильность сложна и её не легко доказать. Система типов Rust несёт огромную часть этого бремени, но система типов не может обнаружить каждый вид некорректности. Таким образом, Rust включает в себя поддержку написания автоматизированных программных тестов внутри языка.

As an example, say we write a function called `add_two` that adds 2 to whatever
number is passed to it. This function’s signature accepts an integer as a
parameter and returns an integer as a result. When we implement and compile
that function, Rust does all the type checking and borrow checking that you’ve
learned so far to ensure that, for instance, we aren’t passing a `String` value
or an invalid reference to this function. But Rust *can’t* check that this
function will do precisely what we intend, which is return the parameter plus 2
rather than, say, the parameter plus 10 or the parameter minus 50! That’s where
tests come in.

Мы можем написать тесты, которые утверждают, например, что когда мы передаём `3` в функцию `add_two`, возвращаемое значение будет `5`. Мы можем запускать эти тесты всякий раз, когда мы вносим изменения в наш код, чтобы убедиться, что любое существующее правильное поведение не изменилось.

Тестирование - сложный навык: хотя мы не можем охватить каждую деталь о написании хороших тестов в одной главе, поэтому мы ещё обсудим механику и возможности тестирования в Rust. Мы поговорим об аннотациях и макросах, доступных вам для написания тестов, о поведении по умолчанию и параметрах, предусмотренных для запуска тестов, как организовать тесты в модульные тесты и интеграционные тесты.
