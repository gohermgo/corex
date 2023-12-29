//! Utilities for comparing and ordering values
//!
//! This module contains 'various' tools for comparing and ordering values. In
//! summary:
//!
//! * [`AlmostEq<Rhs>`] will one day hopefylly overload the `=~` operator once
//!   I figure that out. In cases where `Rhs` (the right hand side's type) is
//!   `Self`, this trait corresponds to an approximate equivalence relation,
//!   based on some `sigma`, which is also of type self.
//!
//! For more details, see the respective documentation
//!

/// Trait for approximate comparison using the =~ some day.
///
/// One day `x.aeq(y, e)` can also be written as x=~(e)y
#[doc(alias = "=~")]
pub trait AlmostEq<Rhs: ?Sized = Self>: PartialOrd<Self> {
    /// This method tests for `self` and `other` values to be approximately
    /// equal, and will maybe one day be used by `=~`.
    #[must_use]
    fn aeq(&self, other: &Rhs, epsilon: Rhs) -> bool;
}
mod impls {
    use crate::cmp::AlmostEq;
    macro_rules! almosteq_impl {
        ($($t:ty)*) => ($(
            impl AlmostEq for $t {
                #[inline]
                fn aeq(&self, other: &$t, epsilon: $t) -> bool {
                    let diff = self - other;
                    if diff.is_sign_negative() {
                        (-1.0 * diff) < epsilon
                    } else {
                        diff < epsilon
                    }
                }
            }
        )*)
    }
    almosteq_impl! { f32 f64 }
}
