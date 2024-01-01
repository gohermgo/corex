//! Trait to handle signability
pub trait Signable {
    /// The signedness of this type
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// assert_eq!(<u8 as Signable>::SIGNED, false);
    /// ```
    const SIGNED: bool;
    /// A function that returns the signedness of this value
    ///
    /// # Exampels
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// assert_eq!(1i32.is_signed(), true);
    /// ```
    fn is_signed(&self) -> bool {
        Self::SIGNED
    }
}
