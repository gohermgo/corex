//! Trait to handle zeroability
pub trait Zeroable: ::core::marker::Sized {
    /// The zero value represented by this type
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// assert_eq!(<Self as Zeroable>::ZERO, 0);
    /// ```
    const ZERO: Self;
    /// A function to return the zero value represented by this type
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// assert_eq!(-5isize.zero(), 0isize);
    /// ```
    fn zero(&self) -> Self {
        Self::ZERO
    }
}
