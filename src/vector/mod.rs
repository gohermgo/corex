//! Utilities for the `Vector` derived type.
//!
//! *[See also the `Vector` derived type](derived@Vector).*
//!
//! The `Vector` type represents <TODO>.
//!
//! [Vector]: https://www.youtube.com/watch?v=KQ6zr6kCPj8&pp=ygUKcGFydHkgcm9jaw%3D%3D

#![allow(non_snake_case, non_camel_case_types)]

type vector<T: core::marker::Sized + corex::ops::BitZero> = [T; 4usize];

mod methods;
// Some modules maybe, like convert, methods, etc

// re-exports
// pub use self::methods yada yada

//// The zero vector
// pub const ZERO: vector<T: BitZero> = vector::ZERO;
