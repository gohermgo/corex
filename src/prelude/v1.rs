//! The first version of the corex prelude.
//!
//! See the [module-level documentation](super) for more.

// Re-exported corex operators
#[doc(no_inline)]
pub use crate::marker::{Numeric, NumericRef, Signed, Unsigned};
#[doc(no_inline)]
pub use crate::ops::{BitOnes, BitZero};

// Re-exported types and traits
#[doc(no_inline)]
pub use crate::cmp::AlmostEq;
