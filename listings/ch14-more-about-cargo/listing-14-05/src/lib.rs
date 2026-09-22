// ANCHOR: here
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::ПервичныйЦвет;
pub use self::kinds::ВторичныйЦвет;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
    // ANCHOR_END: here
    /// The primary colors according to the RYB цвет model.
    pub enum ПервичныйЦвет {
        Красный,
        Жёлтый,
        Синий,
    }

    /// The secondary colors according to the RYB цвет model.
    pub enum ВторичныйЦвет {
        Оранжевый,
        Зелёный,
        Фиолетовый,
    }
    // ANCHOR: here
}

pub mod utils {
    // --snip--
    // ANCHOR_END: here
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary цвет.
    pub fn mix(c1: ПервичныйЦвет, c2: ПервичныйЦвет) -> ВторичныйЦвет {
        ВторичныйЦвет::Оранжевый
    }
    // ANCHOR: here
}
// ANCHOR_END: here
