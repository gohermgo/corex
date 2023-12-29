//! This module reexports the derived types to allow usage that is not
//! possibly shadowed by other declared types.
//!
//! This is normally only useful in macro generated code.
//!
//! An example of this is when generating a new struct and an impl for it:
//!
//! ```rust,compile_fail
//! pub struct Vector;
//!
//! impl QueryId for Vector {
//!     const SOME_PROPERTY: Vector = todo!();
//! }
//!
//! # trait QueryId { const SOME_PROPERTY: corex::derived::Vector; }
//! ```
//!
//! Note that the `SOME_PROPERTY` associated constant would not compile, as its
//! type `Vector` refers to the struct, rather than to the derived `Vector`
//! type.
//!
//! A correct implementation would look like:
//!
//! ```rust,compile_fail
//! pub struct Vector;
//!
//! impl QueryId for Vector {
//!     const SOME_PROPERTY: corex::derived::Vector = todo!();
//! }
//!
//! # trait QueryId { const SOME_PROPERTY: corex::derived::Vector; }
//! ```

pub(crate) use crate::vector;
