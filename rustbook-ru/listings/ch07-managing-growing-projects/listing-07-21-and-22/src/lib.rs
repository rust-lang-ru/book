mod перед_дома;

pub use crate::перед_дома::hosting;

pub fn eat_at_restaurant() {
    hosting::добавить_в_ожидание();
}
