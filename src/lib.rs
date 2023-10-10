#![deny(missing_docs)]
//! [Parity] is a trait for indicating whether a number is odd or even.

/// Provides an interface to check the evenness or oddness of a value.
///
/// Implemented for all primitive numeric types. For integer types `self.is_even()` is equivalent to `!self.is_odd()`.
/// For floating-point types, both [Parity::is_even] and [Parity::is_odd] also require that there is no fractional part
/// in order to return true.
pub trait Parity {
    /// Returns `true` if `self` is even, and false otherwise.
    fn is_even(&self) -> bool;
    /// Returns `true` if `self` is odd, and false otherwise.
    fn is_odd(&self) -> bool;
}

macro_rules! impl_parity {
    ($($T:ty),*) => { $(
        impl Parity for $T {
            /// Returns `true` if the `self` is even.
            ///
            /// For integer types `self.is_even()` is equivalent to `!self.is_odd()`.
            /// # Example
            /// ```
            /// use parity::Parity;
            #[doc = concat!("assert_eq!(2", stringify!($T), ".is_even(), true);")]
            #[doc = concat!("assert_eq!(3", stringify!($T), ".is_even(), false);")]
            /// ```
            #[inline]
            fn is_even(&self) -> bool {
                *self & 1 == 0
            }

            /// Returns `true` if the `self` is odd.
            ///
            /// For integer types `self.is_odd()` is equivalent to `!self.is_even()`.
            /// # Example
            /// ```
            /// use parity::Parity;
            #[doc = concat!("assert_eq!(3", stringify!($T), ".is_odd(), true);")]
            #[doc = concat!("assert_eq!(2", stringify!($T), ".is_odd(), false);")]
            /// ```
            #[inline]
            fn is_odd(&self) -> bool {
                *self & 1 != 0
            }
        }
    )* };
}

macro_rules! impl_float_parity {
    ($($T:ty),*) => { $(
        impl Parity for $T {
            /// Returns `true` if `self` is even and has no fractional part.
            /// # Example
            /// ```
            /// use parity::Parity;
            #[doc = concat!("assert_eq!(2.0", stringify!($T), ".is_even(), true);")]
            #[doc = concat!("assert_eq!(3.0", stringify!($T), ".is_even(), false);")]
            #[doc = concat!("assert_eq!(2.5", stringify!($T), ".is_even(), false);")]
            /// ```
            #[inline]
            fn is_even(&self) -> bool {
                self.fract() == 0.0 && *self % 2.0 == 0.0
            }

            /// Returns `true` if `self` is odd and has no fractional part.
            /// ```
            /// use parity::Parity;
            #[doc = concat!("assert_eq!(3.0", stringify!($T), ".is_odd(), true);")]
            #[doc = concat!("assert_eq!(2.0", stringify!($T), ".is_odd(), false);")]
            #[doc = concat!("assert_eq!(3.5", stringify!($T), ".is_odd(), false);")]
            /// ```
            #[inline]
            fn is_odd(&self) -> bool {
                self.fract() == 0.0 && *self % 2.0 != 0.0
            }
        }
    )* };
}

impl_parity![i8, i16, i32, i64, i128, isize];
impl_parity![u8, u16, u32, u64, u128, usize];
impl_float_parity![f32, f64];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(2.is_even());
        assert!(3.is_odd());

        assert!(2u32.is_even());
        assert!(3u32.is_odd());

        assert!(3i64.is_odd());
        assert!(3u64.is_odd());

        assert!(3i128.is_odd());
        assert!(3u128.is_odd());

        assert!(2f32.is_even());
        assert!(2f64.is_even());
    }

    #[test]
    fn bounds() {
        assert!(0.is_even());
        assert!((-1).is_odd());
        assert!(!(-2).is_odd());
        assert!((-3).is_odd());
        assert!(1.is_odd());
        assert!(u32::MAX.is_odd());
        assert!(u32::MIN.is_even());
        assert!(i32::MIN.is_even());
        assert!(i32::MAX.is_odd());
    }

    #[test]
    fn floats() {
        assert!(!(0.00000001).is_odd());
        assert!(!(0.00000001).is_even());
        assert!(!(1.5).is_odd());
        assert!(!(1.5).is_even());
        assert!(1.0.is_odd());
        assert!(2.0.is_even());

        assert!((-1.0).is_odd());
        assert!((-2.0).is_even());

        assert!(!f32::NAN.is_even());
        assert!(!f32::NAN.is_odd());
        assert!(!f64::NAN.is_even());
        assert!(!f64::NAN.is_odd());
    }
}
