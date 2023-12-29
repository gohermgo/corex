/// The type-agnostic zeroing operator
///
/// # Examples
///
/// An implementation of `Not` for a signed type, hopefully can be enable
/// the use of .- to zero the bits
///
/// ```rust
/// use corex::ops::BitZero;
///
/// assert_eq!(i32::MIN.bitzero(), 0i32);
/// assert_eq!(u32::MAX.bitzero(), u32::MIN);
/// ```
#[rustc_on_unimplemented(
    message = "no implementation for `core::ops::BitXor` for `{Self}`",
    label = "no implementation for `core::ops::BitXor` for `{Self}`",
    append_const_msg
)]
#[doc(alias = ".-")]
pub trait BitZero {
    /// The resulting type after applying the `self ^ self` operator
    type Output;

    /// Performs the `self ^ self` operation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use corex::ops::BitZero;
    ///
    /// assert_eq!(5u8.bitzero(), 0u8);
    /// assert_eq!(16u32.bitzero(), 0u32);
    /// ```
    #[must_use]
    fn bitzero(self) -> Self::Output;
}
/// The type-agnostic 'one'-ing operator
///
/// # Examples
///
/// An implementation of `Not` for a signed type, hopefully can be enable
/// the use of .+ to zero the bits
///
/// ```rust
/// use corex::ops::BitOnes;
///
/// assert_eq!(i32::MAX.bitones(), -1i32);
/// assert_eq!(u32::MIN.bitones(), u32::MAX);
/// ```
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
    /// ```rust
    /// use corex::ops::BitOnes;
    ///
    /// assert_eq!(5u8.bitones(), u8::MAX);
    /// assert_eq!(16u32.bitones(), u32::MAX);
    /// ```
    #[must_use]
    fn bitones(self) -> Self::Output;
}

#[macro_use]
mod impls {
    use {super::BitOnes, super::BitZero, core::arch::asm};
    // Bitzero implementations
    #[cfg(target_pointer_width = "64")]
    macro_rules! bitzero_64_impl {
        ($($t:ty)*) => ($(
            impl BitZero for $t {
                type Output = $t;

                #[inline]
                #[allow(unused_assignments)]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0:r}, {0:r}",
                            out(reg) self
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
                #[allow(unused_assignments)]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0:e}, {0:e}",
                            out(reg) self
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
                #[allow(unused_assignments)]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0:x}, {0:x}",
                            out(reg) self
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
                #[allow(unused_assignments)]
                fn bitzero(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0}, {0}",
                            out(reg_byte) self
                        );
                    };
                    self
                }
            }

            forward_ref_unop! { impl BitZero, bitzero for $t }
        )*)
    }
    bitzero_8_impl! { u8 i8 }
    // Bitones implementations
    #[cfg(target_pointer_width = "64")]
    macro_rules! bitones_64_impl {
        ($($t:ty)*) => ($(
            impl BitOnes for $t {
                type Output = $t;

                #[inline]
                #[allow(unused_assignments)]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0:r}, {0:r}",
                            out(reg) self
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
                #[allow(unused_assignments)]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0:e}, {0:e}",
                            inout(reg) self
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
                #[allow(unused_assignments)]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0:x}, {0:x}",
                            out(reg) self
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
    /// Macro to implement using `reg_byte` in inline assembly
    macro_rules! bitones_8_impl {
        ($($t:ty)*) => ($(
            impl BitOnes for $t {
                type Output = $t;

                #[inline]
                #[allow(unused_assignments)]
                fn bitones(mut self) -> $t {
                    unsafe {
                        asm!(
                            "xor {0}, {0}",
                            out(reg_byte) self
                        );
                    };
                    !self
                }
            }
            // Implements for references of `$t`
            forward_ref_unop! { impl BitOnes, bitones for $t }
        )*)
    }
    // Implementations
    bitones_8_impl! {
        u8
        i8
    }
}

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
