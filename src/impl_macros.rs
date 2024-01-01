use ::corex::num::{sign::Signable, zero::Zeroable};

macro_rules! int_impl {
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
        impl Signable for $SelfT {
            /// The signedness of this type
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            #[doc = concat!("assert_eq!(", stringify($SelfT), "::SIGNED, ", true, ");")]
            /// ```
            const SIGNED: bool = true;
        }
    };
}
macro_rules! uint_impl {
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
            const ZERO: Self = $SelfT::MIN;
        }
        impl Signable for $SelfT {
            /// The signedness of this type
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            #[doc = concat!("assert_eq!(", stringify($SelfT), "::SIGNED, ", false, ");")]
            /// ```
            const SIGNED: bool = false;
        }
    };
}
