//! Overloadable operators i guess
//! # Examples
//!
//! ```rust
//! use corex::ops::{BitOnes, BitZero};
//!
//! assert_eq!(i32::MAX.bitones(), -1i32);
//! assert_eq!(u32::MIN.bitones(), u32::MAX);
//! assert_eq!(u8::MAX.bitzero(), u8::MIN);
//! ```
mod bit;

pub use self::bit::{BitOnes, BitZero};
