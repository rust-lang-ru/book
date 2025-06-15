pub trait Iterator {
    type Предмет;

    fn next(&mut self) -> Option<Self::Item>;
}
