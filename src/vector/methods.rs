//! impl vector {}

use super::*;

impl<T> vector<T> {
    pub const ZERO: vector<T> = [
        corex::ops::BitZero::bitzero,
        corex::ops::BitZero::bitzero,
        corex::ops::BitZero::bitzero,
        corex::ops::BitZero::bitzero,
    ];
}
