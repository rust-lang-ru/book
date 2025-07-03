mod перед_дома {
    pub mod hosting {
        pub fn добавить_в_ожидание() {}
    }
}

use crate::перед_дома::hosting;

pub fn поесть_в_ресторане() {
    hosting::добавить_в_ожидание();
}
