//! Utilities for comparing and ordering values
//!
//! This module contains 'various' tools for comparing and ordering values. In
//! summary:
//!
//! * [`AlmostEq<Rhs>`] will one day hopefylly overload the `=~` operator once
//!   I figure that out. In cases where `Rhs` (the right hand side's type) is
//!   `Self`, this trait corresponds to an approximate equivalence relation,
//!   based on some `epsilon`, which is also of type self.
//!
//! For more details, see the respective documentation
//!

/// Trait for approximate comparison using the =~ some day.
///
/// One day `x.aeq(y, e)` can also be written as x=~(e)y
#[doc(alias = "=~")]
#[rustc_on_unimplemented(
    message = "can't compare `{Self}` with `{Rhs}`",
    label = "no implementation for `{Self} =~ {Rhs}`",
    append_const_msg
)]
#[rustc_diagnostic_item = "AlmostEq"]
pub trait AlmostEq<Rhs: ?core::marker::Sized = Self>: core::cmp::PartialOrd<Self> {
    /// This method tests for `self` and `other` values to be approximately
    /// equal, and will maybe one day be used by `=~`.
    #[must_use]
    fn aeq(&self, other: &Rhs, epsilon: Rhs) -> bool;
}
// Implementation of AlmostEq for primitive types
mod impls {
    use crate::cmp::AlmostEq;
    // macro_rules! almosteq_float_impl {
    //     ($($t:ty)*) => ($(
    //         impl AlmostEq for $t {
    //             #[inline]
    //             fn aeq(&self, other: &$t, epsilon: $t) -> bool {
    //                 let diff = self - other;
    //                 if diff.is_sign_negative() {
    //                     (-1.0 * diff) < epsilon
    //                 } else {
    //                     diff < epsilon
    //                 }
    //             }
    //         }
    //     )*)
    // }
    // #[cfg(target_pointer_width = "64")]
    // almosteq_float_impl! {
    //     f32 f64
    // }
    // #[cfg(target_pointer_width = "32")]
    // almosteq_float_impl! {
    //     f32
    // }
    macro_rules! almosteq_signed_impl {
        ($($t:ty)*) => ($(
            impl AlmostEq for $t {
                #[inline]
                fn aeq(&self, other: &$t, epsilon: $t) -> bool {
                    let diff = self - other;
                    let zero: $t = self - self;
                    if diff < zero {
                        (-diff) < epsilon
                    } else {
                        diff < epsilon
                    }
                }
            }
        )*)
    }
    #[cfg(target_pointer_width = "64")]
    almosteq_signed_impl! {
        i8 i16 i32 i64 i128
        isize
        f32 f64
    }
    #[cfg(target_pointer_width = "32")]
    almosteq_signed_impl! {
        i8 i16 i32
        isize
        f32
    }
    macro_rules! almosteq_unsigned_impl {
        ($($t:ty)*) => ($(
            impl AlmostEq for $t {
                #[inline]
                fn aeq(&self, other: &$t, epsilon: $t) -> bool {
                    let diff = self - other;
                    diff < epsilon
                }
            }
        )*)
    }
    #[cfg(target_pointer_width = "64")]
    almosteq_unsigned_impl! {
        u8 u16 u32 u64 u128
        usize
    }
    #[cfg(target_pointer_width = "32")]
    almosteq_unsigned_impl! {
        u8 u16 u32
        usize
    }
}
