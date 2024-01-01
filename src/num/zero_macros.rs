use ::corex::num::zero::Zeroable;

macro_rules! zero_int_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ty,

        BITS = $BITS:literal,
        Zero = $Zero:literal,
    ) => {
        impl Zeroable for $SelfT {
            /// The zero value represented by this type
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::ZERO, ", stringify!($Zero), ");")]
            /// ```
            const ZERO: Self = !(<$UnsignedT>::MAX) as Self;
        }
    };
}
macro_rules! zero_uint_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        SignedT = $SignedT:ty,

        BITS = $BITS:literal,
        Zero = $Zero:literal,
    ) => {
        impl Zeroable for $SeltT {
            /// The zero value represented by this type
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::ZERO, ", stringify!($Zero), ");")]
            /// ```
            const ZERO: Self = !(<$UnsignedT>::MAX) as Self;
        }
    };
}
