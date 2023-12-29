//! Primitive traits and types representing basic properties of types
//!
//! Rust types can be classified in various useful ways according to
//! their intrinsic properties. These classifications are represented
//! as traits.
//!
//! For those that find that limiting, here are categories for less
//! 'functionality based' grouping.

/// Implements a given marker trait for multiple types at the same time.
///
/// The basic syntax looks like this:
/// ```ignore private macro
/// marker_impls! { MarkerTrait for u8, i8 }
/// ```
/// You can also implement `unsafe` traits
/// ```ignore private macro
/// marker_impls! { unsafe MarkerTrait for u8, i8 }
/// ```
/// Add attributes to all impls:
/// ```ignore private macro
/// marker_impls! {
///     #[allow(lint)]
///     MarkerTrait for u8, i8
/// }
/// ```
/// And use generics:
/// ```ignore private macro
/// marker_impls! {
///     MarkerTrait for
///         u8, i8,
///         {T: ?Sized} *const T,
///         {T: ?Sized} *mut T,
///         {T: MarkerTrait} PhantomData<T>,
///         u32,
/// }
/// ```
macro marker_impls {
    ( $(#[$($meta:tt)*])* $Trait:ident for $({$($bounds:tt)*})? $T:ty $(, $($rest:tt)*)? ) => {
        $(#[$($meta)*])* impl< $($($bounds)*)? > $Trait for $T {}
        marker_impls! { $(#[$($meta)*])* $Trait for $($($rest)*)? }
    },
    ( $(#[$($meta:tt)*])* $Trait:ident for ) => {},

    ( $(#[$($meta:tt)*])* unsafe $Trait:ident for $({$($bounds:tt)*})? $T:ty $(, $($rest:tt)*)? ) => {
        $(#[$($meta)*])* unsafe impl< $($($bounds)*)? > $Trait for $T {}
        marker_impls! { $(#[$($meta)*])* unsafe $Trait for $($($rest)*)? }
    },
    ( $(#[$($meta:tt)*])* unsafe $Trait:ident for ) => {},
}

/// Marker trait for numbers
#[doc(alias = "№", notable_trait)]
#[rustc_diagnostic_item = "Numeric"]
pub trait Numeric {}

/// Marker trait for references to numbers
#[doc(alias = "&№", notable_trait)]
pub trait NumericRef {}

/// Marker trait for signed `Numeric` and `NumericRef` types
#[doc(alias = "(i/f)X")]
pub trait Signed {}

/// Marker trait for unsigned `Numeric` and `NumericRef` types
#[doc(alias = "uX")]
pub trait Unsigned {}

mod impls {
    use super::{marker_impls, Numeric, NumericRef, Signed, Unsigned};
    // Numeric impls
    marker_impls! {
        Numeric for u8, u16, u32, usize, i8, i16, i32, isize, f32
    }
    #[cfg(target_pointer_width = "64")]
    marker_impls! {
        Numeric for u64, u128, i64, i128,  f64
    }
    // NumericReference impls
    marker_impls! {
        Numeric for &u8, &u16, &u32, &usize, &i8, &i16, &i32, &isize, &f32
    }
    #[cfg(target_pointer_width = "64")]
    marker_impls! {
        NumericRef for &u64, &u128, &i64, &i128, &f64
    }
    // macro_rules! numeric_impl {
    //     ($($t:ty)*) => ($(
    //         impl NumericRef for $t {}
    //     )*)
    // }
    // numeric_impl! {
    //     &u8 &u16 &u32 &usize
    //     &i8 &i16 &i32 &isize
    //     &f32
    // }
    // #[cfg(target_pointer_width = "64")]
    // numeric_impl! {
    //     &u64 &u128
    //     &i64 &i128
    //     &f64
    // }

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
