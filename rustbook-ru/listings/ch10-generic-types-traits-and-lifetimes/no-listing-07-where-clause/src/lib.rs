// ANCHOR: here
fn некоторый_способ<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ANCHOR_END: here
    unimplemented!()
}
