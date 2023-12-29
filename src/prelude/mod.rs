//! The corex prelude
//!
//! This module is intended for users of core which do not link to std as well.
//! This module is imported by doing it, dummy RTFM

pub mod v1;

/// 2021 version of the corex prelude
///
/// See the [module-level documentation](self) for more.
pub mod rust_2021 {
    #[doc(no_inline)]
    pub use super::v1::*;

    #[doc(no_inline)]
    pub use core::iter::FromIterator;

    #[doc(no_inline)]
    pub use core::convert::{TryFrom, TryInto};
}
