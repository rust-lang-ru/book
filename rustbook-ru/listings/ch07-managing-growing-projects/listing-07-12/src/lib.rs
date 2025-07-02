mod перед_дома {
    pub mod hosting {
        pub fn добавить_в_ожидание() {}
    }
}

use crate::перед_дома::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::добавить_в_ожидание();
    }
}
