//! Extension numeric traits and functions for the built-in numeric types.

// Used because the `?` is not allowed in a const context
macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return None,
        }
    };
}

mod sign;
mod zero;
// pub trait Zeroable {
//     const ZERO: Self;
// }

#[macro_use]
mod impl_macros; // import int_impl!, uint_impl!, and float_impl!

int_impl!(Self = i8, ActualT = i8, UnsignedT = u8);
int_impl!(Self = i16, ActualT = i16, UnsignedT = u16);
int_impl!(Self = i32, ActualT = i32, UnsignedT = u32);
int_impl!(Self = i64, ActualT = i64, UnsignedT = u64);
int_impl!(Self = i128, ActualT = i128, UnsignedT = u128);

#[cfg(target_pointer_width = "16")]
int_impl!(Self = isize, ActualT = i16, UnsignedT = usize);
#[cfg(target_pointer_width = "32")]
int_impl!(Self = isize, ActualT = i32, UnsignedT = usize);
#[cfg(target_pointer_width = "64")]
int_impl!(Self = isize, ActualT = i64, UnsignedT = usize);

uint_impl!(Self = u8, ActualT = u8, SignedT = u8);
uint_impl!(Self = u16, ActualT = u16, SignedT = u16);
uint_impl!(Self = u32, ActualT = u32, SignedT = u32);
uint_impl!(Self = u64, ActualT = u64, SignedT = u64);
uint_impl!(Self = u128, ActualT = u128, SignedT = u128);

#[cfg(target_pointer_width = "16")]
uint_impl!(Self = usize, ActualT = u16, SignedT = isize);
#[cfg(target_pointer_width = "32")]
uint_impl!(Self = usize, ActualT = u32, SignedT = isize);
#[cfg(target_pointer_width = "64")]
uint_impl!(Self = usize, ActualT = u64, SignedT = isize);

float_impl!(Self = f32, ActualT = f32, UnsignedT = u32);
float_impl!(Self = f64, ActualT = f64, UnsignedT = u64);
