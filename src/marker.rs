//! Primitive traits and types representing basic properties of types
//!
//! Rust types can be classified in various useful ways according to
//! their intrinsic properties. These classifications are represented
//! as traits.
//!
//! For those that find that limiting, here are categories for less
//! 'functionality based' grouping.

/// Marker trait for numbers
#[doc(alias = "№")]
pub trait Numeric {}

/// Marker trait for references to numbers
#[doc(alias = "&№")]
pub trait NumericRef {}

/// Marker trait for signed `Numeric` and `NumericRef` types
#[doc(alias = "(i/f)X")]
pub trait Signed {}

/// Marker trait for unsigned `Numeric` and `NumericRef` types
#[doc(alias = "uX")]
pub trait Unsigned {}

#[macro_use]
mod impls {
    use super::{Numeric, NumericRef, Signed, Unsigned};
    // Numeric impls
    macro_rules! numeric_impl {
        ($($t:ty)*) => ($(
            impl Numeric for $t {}
        )*)
    }
    numeric_impl! {
        u8 u16 u32 usize
        i8 i16 i32 isize
        f32
    }
    #[cfg(target_pointer_width = "64")]
    numeric_impl! {
        u64 u128
        i64 i128
        f64
    }
    // NumericReference impls
    macro_rules! numeric_impl {
        ($($t:ty)*) => ($(
            impl NumericRef for $t {}
        )*)
    }
    numeric_impl! {
        &u8 &u16 &u32 &usize
        &i8 &i16 &i32 &isize
        &f32
    }
    #[cfg(target_pointer_width = "64")]
    numeric_impl! {
        &u64 &u128
        &i64 &i128
        &f64
    }

    macro_rules! signed_impl {
        ($($t:ty)*) => ($(
            impl Signed for $t {}
            // Implements for references
            forward_ref_noop! { impl Signed for $t }
        )*)
    }
    signed_impl! {
        i8 i16 i32 isize
        f32
    }
    #[cfg(target_pointer_width = "64")]
    signed_impl! {
        i64 i128
        f64
    }
    macro_rules! unsigned_impl {
        ($($t:ty)*) => ($(
            impl Unsigned for $t {}
            // Implements for references
            forward_ref_noop! { impl Unsigned for $t }
        )*)
    }
    unsigned_impl! {
        u8 u16 u32 usize
    }
    #[cfg(target_pointer_width = "64")]
    unsigned_impl! {
        u64 u128
    }
}
