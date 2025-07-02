mod перед_дома {
    pub mod hosting {
        pub fn добавить_в_ожидание() {}
    }
}

use crate::перед_дома::hosting::добавить_в_ожидание;

pub fn eat_at_restaurant() {
    добавить_в_ожидание();
}
