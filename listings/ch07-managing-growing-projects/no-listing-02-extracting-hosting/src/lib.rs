mod перед_дома;

pub use crate::перед_дома::hosting;

pub fn поесть_в_ресторане() {
    hosting::добавить_в_ожидание();
}
