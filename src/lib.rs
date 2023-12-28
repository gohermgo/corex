#![no_std]
#![allow(internal_features)]
#![feature(
    core_io_borrowed_buf,
    lang_items,
    trait_alias,
    rustc_attrs,
    internal_impls_macro
)]
mod zeroable {}

mod marker {
    use core::ops::{BitXor, Not};
    pub(super) trait BitMutable {}
    impl<N> BitMutable for N
    where
        N: Not<Output = N>,
        for<'a> N: 'a,
        for<'a> &'a N: BitXor<&'a N, Output = N>,
    {
    }
    // impl<'a, N> BitMutable for &N where &'a N: BitXor<&'a N, Output = N> {}
}

mod internal_macros {
    // implements binary operators "&T op U", "T op &U", "&T op &U"
    // based on "T op U" where T and U are expected to be `Copy`able
    macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        forward_ref_binop!(impl $imp, $method for $t, $u,
                #[stable(feature = "rust1", since = "1.0.0")]);
    };
    (impl $imp:ident, $method:ident for $t:ty, $u:ty, #[$attr:meta]) => {
        #[$attr]
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        #[$attr]
        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        #[$attr]
        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}
}

mod bits {
    use core::arch::asm;
    #[rustc_on_unimplemented(
        message = "no implementation for `core::ops::BitXor` for `{Self}`",
        label = "no implementation for `core::ops::BitXor` for `{Self}`",
        append_const_msg
    )]
    #[doc(alias = "~")]
    pub trait BitZero {
        /// The resulting type after applying the `self ^ self` operator
        type Output;

        /// Performs the `self ^ self` operation
        ///
        /// # Examples
        ///
        /// ```
        /// assert_eq!(5u8.bitzero(), 0u8);
        /// assert_eq!(16u32.bitzero(), 0u32);
        /// ```
        #[must_use]
        fn bitzero(self) -> Self::Output;
    }
    macro_rules! bitzero_impl {
        ($($t:ty)*) => ($(
            impl BitZero for $t {
                type Output = $t;

                #[inline]
                fn bitzero(self) -> $t {
                    unsafe {
                        asm!(
                            "xor {t} {t}",
                            x = inout(reg) self
                        );
                    };
                    self
                }
            }
        )*)
    }
    use crate::marker::*;
    /// Turns any type [`&Self`] implementing [`core::ops::BitXor<&Self, Output = Self>`] into zeroes regardless of bit count
    pub(super) const fn all_zero<N>(value: &N) -> N
    where
        for<'a> N: 'a,
    {
        asm!("xor {value} {value}", value = inout(reg) value)
        // value ^ value
    }

    /// Turns any type [`&Self`] implementing [`Zeroable`] and [`core::ops::Not<Output = Self>`] into ones regardless of bitcount
    pub(super) const fn all_ones<N: BitMutable>(value: &N) -> N {
        !(value ^ value)
    }

    /// Checks if any type is signed
    pub(super) const fn is_signed<N: BitOps>(value: &N) -> bool
    where
        N: core::cmp::PartialOrd,
    {
        value.all_ones() < value.all_zero()
    }
    pub trait BitOps: BitMutable
    where
        Self: Sized,
    {
        fn all_zero(&self) -> Self {
            all_zero(self)
        }

        fn all_ones(&self) -> Self {
            all_ones(self)
        }
    }
    // pub(super) trait OneMarker: Zeroable {}
    // impl<Numeric> OneMarker for Numeric
    // where
    //     Numeric: ZeroMarker,
    //     for<'a> Numeric: 'a,
    //     for<'a> &'a Numeric: Not<Output = Numeric>,
    // {
    // }

    /// Checks if any type implementing [`BitOps`] is signed
    // pub(super) const fn is_signed<Numeric: OneMarker>(value: &Numeric) -> bool {
    //     value.zero()
    // }

    #[cfg(test)]
    mod tests {
        use super::BitOps;

        // Zero tests
        #[test]
        fn zero() {
            let zero = usize::MAX.all_zero();
            assert_eq!(zero, usize::MIN);
            let zero = u8::MAX.zero();
            assert_eq!(zero, u8::MIN);
            let zero = u16::MAX.zero();
            assert_eq!(zero, u16::MIN);
            let zero = u32::MAX.zero();
            assert_eq!(zero, u32::MIN);
            let zero = u64::MAX.zero();
            assert_eq!(zero, u64::MIN);
            let zero = u128::MAX.zero();
            assert_eq!(zero, u128::MIN);
        }
        // One tests
        // Signed tests
        // Unsigned tests
    }
}

// pub trait BitOps<'a>
// where
//     Self: 'a + Sized + Zeroable<'a> + core::ops::Not<Output = Self>,
// {
//     fn all_ones(&self) -> Self {
//         bits::all_ones(self)
//     }
// }

// impl<'a, T> BitOps<'a> for T where T: 'a + Zeroable<'a> {}

// mod num {
//     use {
//         crate::{bits::all_ones, zeroable::zero, Zeroable},
//         ::core::ops::{BitXor, Not},
//     };
//     pub(super) const fn is_signed<'a, T: Zeroable<'a>>(value: &T) -> T
//     where
//         T: 'a + Not<Output = T>,
//     {
//     }
// }

// impl<T> PrimitiveExt<T> for T where
//     T: Copy
//         + ::core::ops::BitXor<Output = T>
//         + ::core::ops::Not<Output = T>
//         + ::core::cmp::PartialOrd
// {
// }

// pub fn abs_diff<
//     T: Copy + core::ops::BitXor<Output = T> + core::ops::Not<Output = T> + core::cmp::PartialOrd,
// >(
//     this: T,
//     other: T,
// ) -> T
// where
//     T: PrimitiveExt<T>,
// {
// }

pub fn equal<T>(this: T, other: T) -> bool
where
    T: ::core::cmp::Ord,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
