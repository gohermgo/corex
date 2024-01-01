//! # The Rust Core Extension Library
//!
//! Still dependency free, and just features some extra nice bits
// This cfg won't affect doc tests.
// #![cfg(not(test))]
#![cfg_attr(not(bootstrap), doc(rust_logo))]
// #![doc(cfg_hide(
//     not(test),
//     np_fp_fmt_parse,
//     target_pointer_width = "16",
//     target_pointer_width = "32",
//     target_pointer_width = "64",
//     target_has_atomic = "8",
//     target_has_atomic = "16",
//     target_has_atomic = "32",
//     target_has_atomic = "64",
//     target_has_atomic = "ptr",
//     target_has_atomic_equal_alignment = "8",
//     target_has_atomic_equal_alignment = "16",
//     target_has_atomic_equal_alignment = "32",
//     target_has_atomic_equal_alignment = "64",
//     target_has_atomic_equal_alignment = "ptr",
//     target_has_atomic_load_store = "8",
//     target_has_atomic_load_store = "16",
//     target_has_atomic_load_store = "32",
//     target_has_atomic_load_store = "64",
//     target_has_atomic_load_store = "ptr",
// ))]
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
// Library features
// tidy-alphabetical-start
// #![feature(char_indices_offset)]
// #![feature(const_align_of_val)]
// #![feature(const_align_of_val_raw)]
// #![feature(const_align_offset)]
// #![feature(const_alloc_layout)]
// #![feature(const_array_from_ref)]
// #![feature(const_array_into_iter_constructors)]
// #![feature(const_assume)]
// #![feature(const_bigint_helper_methods)]
// #![feature(const_black_box)]
// #![feature(const_caller_location)]
// #![feature(const_cell_into_inner)]
// #![feature(const_char_from_u32_unchecked)]
// #![feature(const_eval_select)]
// #![feature(const_exact_div)]
// #![feature(const_float_bits_conv)]
// #![feature(const_float_classify)]
// #![feature(const_fmt_arguments_new)]
// #![feature(const_hash)]
// #![feature(const_heap)]
// #![feature(const_index_range_slice_index)]
// #![feature(const_int_unchecked_arith)]
// #![feature(const_intrinsic_forget)]
// #![feature(const_ipv4)]
// #![feature(const_ipv6)]
// #![feature(const_likely)]
// #![feature(const_maybe_uninit_as_mut_ptr)]
// #![feature(const_maybe_uninit_assume_init)]
// #![feature(const_maybe_uninit_uninit_array)]
// #![feature(const_nonnull_new)]
// #![feature(const_num_midpoint)]
// #![feature(const_option)]
// #![feature(const_option_ext)]
// #![feature(const_pin)]
// #![feature(const_pointer_is_aligned)]
// #![feature(const_ptr_as_ref)]
// #![feature(const_ptr_is_null)]
// #![feature(const_ptr_sub_ptr)]
// #![feature(const_ptr_write)]
// #![feature(const_raw_ptr_comparison)]
// #![feature(const_replace)]
// #![feature(const_size_of_val)]
// #![feature(const_size_of_val_raw)]
// #![feature(const_slice_from_raw_parts_mut)]
// #![feature(const_slice_from_ref)]
// #![feature(const_slice_index)]
// #![feature(const_slice_ptr_len)]
// #![feature(const_slice_split_at_mut)]
// #![feature(const_str_from_utf8_unchecked_mut)]
// #![feature(const_swap)]
// #![feature(const_try)]
// #![feature(const_type_id)]
// #![feature(const_type_name)]
// #![feature(const_unicode_case_lookup)]
// #![feature(const_unsafecell_get_mut)]
// #![feature(const_waker)]
// #![feature(core_panic)]
// #![feature(coverage_attribute)]
// #![feature(duration_consts_float)]
// #![feature(internal_impls_macro)]
// #![feature(ip)]
// #![feature(ip_bits)]
// #![feature(is_ascii_octdigit)]
// #![feature(isqrt)]
// #![feature(maybe_uninit_uninit_array)]
// #![feature(ptr_alignment_type)]
// #![feature(ptr_metadata)]
// #![feature(set_ptr_value)]
// #![feature(slice_ptr_get)]
// #![feature(slice_split_at_unchecked)]
// #![feature(str_internals)]
// #![feature(str_split_inclusive_remainder)]
// #![feature(str_split_remainder)]
// #![feature(strict_provenance)]
// #![feature(unchecked_math)]
// #![feature(unchecked_shifts)]
// #![feature(utf16_extra)]
// #![feature(utf16_extra_const)]
// #![feature(variant_count)]
// tidy-alphabetical-end
//
// Language features:
// tidy-alphabetical-start
#![feature(
    allow_internal_unsafe,
    allow_internal_unstable,
    auto_traits,
    associated_type_bounds,
    core_io_borrowed_buf,
    decl_macro,
    doc_cfg,
    doc_cfg_hide,
    doc_notable_trait,
    fundamental,
    intrinsics,
    lang_items,
    trait_alias,
    multiple_supertrait_upcastable,
    prelude_import,
    rustc_attrs,
    rustdoc_internals,
    strict_provenance
)]
// allow using `corex::` in intra-doc links
#[allow(unused_extern_crates)]
extern crate self as corex;

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[macro_use]
mod internal_macros;

#[macro_use]
pub mod num;

/* The corex prelude, builds on core */

pub mod prelude;

/* Corex language traits */

pub mod cmp;
pub mod marker;
pub mod ops;

// note: does not need to be public
mod vector;

pub mod derived;

include!("derived_docs.rs");
