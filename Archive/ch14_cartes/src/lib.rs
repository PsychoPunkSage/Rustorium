//! # Ch14-Crates
//!
//! `ch14_crates` ia a collection of test utilities to perform certain calculations easily.

/// Add one to number given.
///
/// # Examples
///
/// ```
/// let a = 5;
/// let ans = ch14_cartes::add_one(a);
/// assert_eq!(ans,6);
/// ```
pub use self::utils::*;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// Primary colors according to RYB
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary colors according to RYB
    pub enum SecondaryColor {
        Green,
        Orange,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combine 2 primary colors in equal amount to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
