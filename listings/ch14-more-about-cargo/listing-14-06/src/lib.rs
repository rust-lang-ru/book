//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::ПервичныйЦвет;
pub use self::kinds::ВторичныйЦвет;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB цветmodel.
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
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary цвет.
    pub fn mix(c1: ПервичныйЦвет, c2: ПервичныйЦвет) -> ВторичныйЦвет {
        ВторичныйЦвет::Оранжевый
    }
}
