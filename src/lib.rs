//! # The Rust Core Extension Library
//!
//! Still dependency free, and just features some extra nice bits
// This cfg won't affect doc tests.
#![cfg(not(test))]
#![cfg_attr(not(bootstrap), doc(rust_logo))]
#![doc(cfg_hide(
    not(test),
    np_fp_fmt_parse,
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_has_atomic = "8",
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
    target_has_atomic = "ptr",
    target_has_atomic_equal_alignment = "8",
    target_has_atomic_equal_alignment = "16",
    target_has_atomic_equal_alignment = "32",
    target_has_atomic_equal_alignment = "64",
    target_has_atomic_equal_alignment = "ptr",
    target_has_atomic_load_store = "8",
    target_has_atomic_load_store = "16",
    target_has_atomic_load_store = "32",
    target_has_atomic_load_store = "64",
    target_has_atomic_load_store = "ptr",
))]
#![no_std]
#![rustc_coherence_is_core]
//
// Lints:
#![deny(
    rust_2021_incompatible_or_patterns,
    unsafe_op_in_unsafe_fn,
    fuzzy_provenance_casts
)]
#![warn(
    deprecated_in_future,
    missing_debug_implementations,
    missing_docs,
    multiple_supertrait_upcastable
)]
#![allow(
    explicit_outlives_requirements,
    incomplete_features,
    internal_features,
    // Do not check link reduncancy on bootstrapping phase
    rustdoc::redundant_explicit_links
)]
#![feature(
    associated_type_bounds,
    core_io_borrowed_buf,
    doc_cfg,
    doc_cfg_hide,
    lang_items,
    trait_alias,
    multiple_supertrait_upcastable,
    rustc_attrs,
    rustdoc_internals,
    strict_provenance,
    internal_impls_macro
)]

// allow using `corex::` in intra-doc links
extern crate self as corex;

#[macro_use]
mod internal_macros;

mod marker;

/* Corex language traits */

pub mod cmp;
pub mod ops;
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
