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

// mod marker {
//     use core::ops::{BitXor, Not};
//     pub(super) trait BitMutable {}
//     impl<N> BitMutable for N
//     where
//         N: Not<Output = N>,
//         for<'a> N: 'a,
//         for<'a> &'a N: BitXor<&'a N, Output = N>,
//     {
//     }
//     // impl<'a, N> BitMutable for &N where &'a N: BitXor<&'a N, Output = N> {}
// }

#[macro_use]
mod internal_macros {
    // implements marker for "&T"
    // based on "T" where T is expected to be `Copy`able
    macro_rules! forward_ref_noop {
        (impl $imp:ident for $t:ty) => {
            impl $imp for &$t {}
        };
    }
    // implements the unary operator "op &T"
    // based on "op T" where T is expected to be `Copy`able
    macro_rules! forward_ref_unop {
        // (impl $imp:ident, $method:ident for $t:ty) => {
        //     forward_ref_unop!(impl $imp, $method for $t);
        // };
        (impl $imp:ident, $method:ident for $t:ty) => {
            // #[$attr]
            impl $imp for &$t {
                type Output = <$t as $imp>::Output;

                #[inline]
                fn $method(self) -> <$t as $imp>::Output {
                    $imp::$method(*self)
                }
            }
        };
    }
    // implements binary operators "&T op U", "T op &U", "&T op &U"
    // based on "T op U" where T and U are expected to be `Copy`able
    // macro_rules! forward_ref_binop {
    //     (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
    //         forward_ref_binop!(impl $imp, $method for $t, $u,
    //                 #[stable(feature = "rust1", since = "1.0.0")]);
    //     };
    //     (impl $imp:ident, $method:ident for $t:ty, $u:ty, #[$attr:meta]) => {
    //         #[$attr]
    //         impl<'a> $imp<$u> for &'a $t {
    //             type Output = <$t as $imp<$u>>::Output;

    //             #[inline]
    //             fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
    //                 $imp::$method(*self, other)
    //             }
    //         }

    //         #[$attr]
    //         impl $imp<&$u> for $t {
    //             type Output = <$t as $imp<$u>>::Output;

    //             #[inline]
    //             fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
    //                 $imp::$method(self, *other)
    //             }
    //         }

    //         #[$attr]
    //         impl $imp<&$u> for &$t {
    //             type Output = <$t as $imp<$u>>::Output;

    //             #[inline]
    //             fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
    //                 $imp::$method(*self, *other)
    //             }
    //         }
    //     }
    // }
}

mod marker {
    #[doc(alias = "№")]
    /// Marker trait for numbers
    pub trait Numeric {}
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
    #[doc(alias = "&№")]
    /// Marker trait for references to numbers
    pub trait NumericReference {}
    macro_rules! numeric_impl {
        ($($t:ty)*) => ($(
            impl NumericReference for $t {}
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
    #[doc(alias = "(i/f)X")]
    /// Marker trait for signed numbers
    pub trait Signed {}
    macro_rules! signed_impl {
        ($($t:ty)*) => ($(
            impl Signed for $t {}
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
    #[doc(alias = "uX")]
    pub trait Unsigned {}
    macro_rules! unsigned_impl {
        ($($t:ty)*) => ($(
            impl Unsigned for $t {}
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

#[macro_use]
mod bits {
    use core::arch::asm;
    #[rustc_on_unimplemented(
        message = "no implementation for `core::ops::BitXor` for `{Self}`",
        label = "no implementation for `core::ops::BitXor` for `{Self}`",
        append_const_msg
    )]
    #[doc(alias = "z~")]
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
    #[cfg(target_pointer_width = "64")]
    macro_rules! bitzero_64_impl {
        ($($t:ty)*) => ($(
            impl BitZero for $t {
                type Output = $t;

                #[inline]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x:r} {x:r}",
                            x = inout(reg) self
                        );
                    };
                    self
                }
            }

            forward_ref_unop! { impl BitZero, bitzero for $t }
        )*)
    }
    #[cfg(target_pointer_width = "64")]
    bitzero_64_impl! {
        u64 usize
        i64 isize
        f64
    }
    macro_rules! bitzero_32_impl {
        ($($t:ty)*) => ($(
            impl BitZero for $t {
                type Output = $t;

                #[inline]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x:e} {x:e}",
                            x = inout(reg) self
                        );
                    };
                    self
                }
            }

            forward_ref_unop! { impl BitZero, bitzero for $t }
        )*)
    }
    bitzero_32_impl! {
        u32
        i32
        f32
    }
    #[cfg(target_pointer_width = "32")]
    bitzero_32_impl! {
        usize
        isize
    }
    macro_rules! bitzero_16_impl {
        ($($t:ty)*) => ($(
            impl BitZero for $t {
                type Output = $t;

                #[inline]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x:x} {x:x}",
                            x = inout(reg) self
                        );
                    };
                    self
                }
            }

            forward_ref_unop! { impl BitZero, bitzero for $t }
        )*)
    }
    bitzero_16_impl! { u16 i16 }
    macro_rules! bitzero_8_impl {
        ($($t:ty)*) => ($(
            impl BitZero for $t {
                type Output = $t;

                #[inline]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x} {x}",
                            x = inout(reg_byte) self
                        );
                    };
                    self
                }
            }

            forward_ref_unop! { impl BitZero, bitzero for $t }
        )*)
    }
    bitzero_8_impl! { u8 i8 }
    #[rustc_on_unimplemented(
        message = "no implementation for `core::ops::Not` for `{Self}`",
        label = "no implementation for `core::ops::Not` for `{Self}`",
        append_const_msg
    )]
    #[doc(alias = "o~")]
    pub trait BitOnes {
        /// The resulting type after applying the `self ^ self` operator
        type Output;

        /// Performs the `self ^ self` operation
        ///
        /// # Examples
        ///
        /// ```
        /// assert_eq!(5u8.bitones(), u8::MAX);
        /// assert_eq!(16u32.bitbones(), u32::max);
        /// ```
        #[must_use]
        fn bitones(self) -> Self::Output;
    }
    #[cfg(target_pointer_width = "64")]
    macro_rules! bitones_64_impl {
        ($($t:ty)*) => ($(
            impl BitOnes for $t {
                type Output = $t;

                #[inline]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x:r} {x:r}",
                            x = inout(reg) self
                        );
                    };
                    !self
                }
            }

            forward_ref_unop! { impl BitOnes, bitones for $t }
        )*)
    }
    #[cfg(target_pointer_width = "64")]
    bitones_64_impl! {
        u64 usize
        i64 isize
    }
    impl BitOnes for f64 {
        type Output = f64;
        fn bitones(self) -> Self::Output {
            !f64::to_bits(self).bitzero() as Self::Output
        }
    }
    macro_rules! bitones_32_impl {
        ($($t:ty)*) => ($(
            impl BitOnes for $t {
                type Output = $t;

                #[inline]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x:e} {x:e}",
                            x = inout(reg) self
                        );
                    };
                    !self
                }
            }

            forward_ref_unop! { impl BitOnes, bitones for $t }
        )*)
    }
    bitones_32_impl! {
        u32
        i32
    }
    #[cfg(target_pointer_width = "32")]
    bitones_32_impl! {
        usize,
        isize
    }
    impl BitOnes for f32 {
        type Output = f32;
        fn bitones(self) -> Self::Output {
            !f32::to_bits(self).bitzero() as Self::Output
        }
    }
    macro_rules! bitones_16_impl {
        ($($t:ty)*) => ($(
            impl BitOnes for $t {
                type Output = $t;

                #[inline]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x:x} {x:x}",
                            x = inout(reg) self
                        );
                    };
                    !self
                }
            }

            forward_ref_unop! { impl BitOnes, bitones for $t }
        )*)
    }
    bitones_16_impl! {
        u16
        i16
    }
    macro_rules! bitones_8_impl {
        ($($t:ty)*) => ($(
            impl BitOnes for $t {
                type Output = $t;

                #[inline]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {x} {x}",
                            x = inout(reg_byte) self
                        );
                    };
                    !self
                }
            }

            forward_ref_unop! { impl BitOnes, bitones for $t }
        )*)
    }
    bitones_8_impl! { u8 i8 }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Zero tests
        #[test]
        fn zero() {
            let zero = usize::MAX.bitzero();
            assert_eq!(zero, usize::MIN);
            let zero = u8::MAX.bitzero();
            assert_eq!(zero, u8::MIN);
            let zero = f32::MAX.bitzero();
            assert_eq!(zero, f32::MIN);
        }
        // One tests
        #[test]
        fn ones() {
            let ones = usize::MIN.bitones();
            assert_eq!(ones, usize::MAX);
            let ones = u8::MIN.bitones();
            assert_eq!(ones, u8::MAX);
            let ones = i16::MAX.bitones();
            assert_eq!(ones, i16::MIN);
        }
        // Signed tests
        // Unsigned tests
    }
}
mod ops {
    #[doc(alias = "=~")]
    pub trait AlmostEq<Rhs: ?Sized = Self>
    where
        Self: Sized,
    {
        #[must_use]
        fn aeq(self, other: Rhs, sigma: Rhs) -> bool
        where
            Self: core::ops::Sub + core::cmp::PartialOrd;
    }
    impl AlmostEq for f32 {
        fn aeq(self, other: Self, sigma: Self) -> bool
        where
            Self: core::ops::Sub + core::cmp::PartialOrd,
        {
            let diff = self - other;
            let abs_diff = if diff.is_sign_negative() {
                -1.0f32 * diff
            } else {
                diff
            };
            abs_diff < sigma
        }
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

// pub fn equal<T>(this: T, other: T) -> bool
// where
//     T: ::core::cmp::Ord,
// {
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
