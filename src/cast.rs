use core::{i8, i16, i32, i64, isize};
use core::{u8, u16, u32, u64, usize};
use core::{f32, f64};
use core::mem::size_of;
use core::num::Wrapping;
#[cfg(feature = "i128")]
use core::{i128, u128};

use float::FloatCore;

/// A generic trait for converting a value to a number.
pub trait ToPrimitive {
    /// Converts the value of `self` to an `isize`.
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.to_i64().and_then(|x| x.to_isize())
    }

    /// Converts the value of `self` to an `i8`.
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.to_i64().and_then(|x| x.to_i8())
    }

    /// Converts the value of `self` to an `i16`.
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i64().and_then(|x| x.to_i16())
    }

    /// Converts the value of `self` to an `i32`.
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i64().and_then(|x| x.to_i32())
    }

    /// Converts the value of `self` to an `i64`.
    fn to_i64(&self) -> Option<i64>;

    /// Converts the value of `self` to an `i128`.
    ///
    /// This method is only available with feature `i128` enabled on Rust >= 1.26.
    ///
    /// The default implementation converts through `to_i64()`.  Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    #[cfg(feature = "i128")]
    fn to_i128(&self) -> Option<i128> {
        self.to_i64().map(From::from)
    }

    /// Converts the value of `self` to a `usize`.
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.to_u64().and_then(|x| x.to_usize())
    }

    /// Converts the value of `self` to an `u8`.
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.to_u64().and_then(|x| x.to_u8())
    }

    /// Converts the value of `self` to an `u16`.
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.to_u64().and_then(|x| x.to_u16())
    }

    /// Converts the value of `self` to an `u32`.
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.to_u64().and_then(|x| x.to_u32())
    }

    /// Converts the value of `self` to an `u64`.
    #[inline]
    fn to_u64(&self) -> Option<u64>;

    /// Converts the value of `self` to an `u128`.
    ///
    /// This method is only available with feature `i128` enabled on Rust >= 1.26.
    ///
    /// The default implementation converts through `to_u64()`.  Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    #[cfg(feature = "i128")]
    fn to_u128(&self) -> Option<u128> {
        self.to_u64().map(From::from)
    }

    /// Converts the value of `self` to an `f32`.
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.to_f64().and_then(|x| x.to_f32())
    }

    /// Converts the value of `self` to an `f64`.
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.to_i64().and_then(|x| x.to_f64())
    }
}

macro_rules! impl_to_primitive_int_to_int {
    ($SrcT:ident : $( $(#[$cfg:meta])* fn $method:ident -> $DstT:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$DstT> {
            let min = $DstT::MIN as $SrcT;
            let max = $DstT::MAX as $SrcT;
            if size_of::<$SrcT>() <= size_of::<$DstT>() || (min <= *self && *self <= max) {
                Some(*self as $DstT)
            } else {
                None
            }
        }
    )*}
}

macro_rules! impl_to_primitive_int_to_uint {
    ($SrcT:ident : $( $(#[$cfg:meta])* fn $method:ident -> $DstT:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$DstT> {
            let max = $DstT::MAX as $SrcT;
            if 0 <= *self && (size_of::<$SrcT>() <= size_of::<$DstT>() || *self <= max) {
                Some(*self as $DstT)
            } else {
                None
            }
        }
    )*}
}

macro_rules! impl_to_primitive_int {
    ($T:ident) => (
        impl ToPrimitive for $T {
            impl_to_primitive_int_to_int! { $T:
                fn to_isize -> isize;
                fn to_i8 -> i8;
                fn to_i16 -> i16;
                fn to_i32 -> i32;
                fn to_i64 -> i64;
                #[cfg(feature = "i128")]
                fn to_i128 -> i128;
            }

            impl_to_primitive_int_to_uint! { $T:
                fn to_usize -> usize;
                fn to_u8 -> u8;
                fn to_u16 -> u16;
                fn to_u32 -> u32;
                fn to_u64 -> u64;
                #[cfg(feature = "i128")]
                fn to_u128 -> u128;
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> { Some(*self as f32) }
            #[inline]
            fn to_f64(&self) -> Option<f64> { Some(*self as f64) }
        }
    )
}

impl_to_primitive_int!(isize);
impl_to_primitive_int!(i8);
impl_to_primitive_int!(i16);
impl_to_primitive_int!(i32);
impl_to_primitive_int!(i64);
#[cfg(feature = "i128")]
impl_to_primitive_int!(i128);

macro_rules! impl_to_primitive_uint_to_int {
    ($SrcT:ident : $( $(#[$cfg:meta])* fn $method:ident -> $DstT:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$DstT> {
            let max = $DstT::MAX as $SrcT;
            if size_of::<$SrcT>() < size_of::<$DstT>() || *self <= max {
                Some(*self as $DstT)
            } else {
                None
            }
        }
    )*}
}

macro_rules! impl_to_primitive_uint_to_uint {
    ($SrcT:ident : $( $(#[$cfg:meta])* fn $method:ident -> $DstT:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$DstT> {
            let max = $DstT::MAX as $SrcT;
            if size_of::<$SrcT>() <= size_of::<$DstT>() || *self <= max {
                Some(*self as $DstT)
            } else {
                None
            }
        }
    )*}
}

macro_rules! impl_to_primitive_uint {
    ($T:ident) => (
        impl ToPrimitive for $T {
            impl_to_primitive_uint_to_int! { $T:
                fn to_isize -> isize;
                fn to_i8 -> i8;
                fn to_i16 -> i16;
                fn to_i32 -> i32;
                fn to_i64 -> i64;
                #[cfg(feature = "i128")]
                fn to_i128 -> i128;
            }

            impl_to_primitive_uint_to_uint! { $T:
                fn to_usize -> usize;
                fn to_u8 -> u8;
                fn to_u16 -> u16;
                fn to_u32 -> u32;
                fn to_u64 -> u64;
                #[cfg(feature = "i128")]
                fn to_u128 -> u128;
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> { Some(*self as f32) }
            #[inline]
            fn to_f64(&self) -> Option<f64> { Some(*self as f64) }
        }
    )
}

impl_to_primitive_uint!(usize);
impl_to_primitive_uint!(u8);
impl_to_primitive_uint!(u16);
impl_to_primitive_uint!(u32);
impl_to_primitive_uint!(u64);
#[cfg(feature = "i128")]
impl_to_primitive_uint!(u128);

macro_rules! impl_to_primitive_float_to_float {
    ($SrcT:ident : $( fn $method:ident -> $DstT:ident ; )*) => {$(
        #[inline]
        fn $method(&self) -> Option<$DstT> {
            // Only finite values that are reducing size need to worry about overflow.
            if size_of::<$SrcT>() > size_of::<$DstT>() && FloatCore::is_finite(*self) {
                let n = *self as f64;
                if n < $DstT::MIN as f64 || n > $DstT::MAX as f64 {
                    return None;
                }
            }
            // We can safely cast NaN, +-inf, and finite values in range.
            Some(*self as $DstT)
        }
    )*}
}

macro_rules! impl_to_primitive_float_to_signed_int {
    ($f:ident : $( $(#[$cfg:meta])* fn $method:ident -> $i:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$i> {
            // Float as int truncates toward zero, so we want to allow values
            // in the exclusive range `(MIN-1, MAX+1)`.
            if size_of::<$f>() > size_of::<$i>() {
                // With a larger size, we can represent the range exactly.
                const MIN_M1: $f = $i::MIN as $f - 1.0;
                const MAX_P1: $f = $i::MAX as $f + 1.0;
                if *self > MIN_M1 && *self < MAX_P1 {
                    return Some(*self as $i);
                }
            } else {
                // We can't represent `MIN-1` exactly, but there's no fractional part
                // at this magnitude, so we can just use a `MIN` inclusive boundary.
                const MIN: $f = $i::MIN as $f;
                // We can't represent `MAX` exactly, but it will round up to exactly
                // `MAX+1` (a power of two) when we cast it.
                const MAX_P1: $f = $i::MAX as $f;
                if *self >= MIN && *self < MAX_P1 {
                    return Some(*self as $i);
                }
            }
            None
        }
    )*}
}

macro_rules! impl_to_primitive_float_to_unsigned_int {
    ($f:ident : $( $(#[$cfg:meta])* fn $method:ident -> $u:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$u> {
            // Float as int truncates toward zero, so we want to allow values
            // in the exclusive range `(-1, MAX+1)`.
            if size_of::<$f>() > size_of::<$u>() {
                // With a larger size, we can represent the range exactly.
                const MAX_P1: $f = $u::MAX as $f + 1.0;
                if *self > -1.0 && *self < MAX_P1 {
                    return Some(*self as $u);
                }
            } else {
                // We can't represent `MAX` exactly, but it will round up to exactly
                // `MAX+1` (a power of two) when we cast it.
                // (`u128::MAX as f32` is infinity, but this is still ok.)
                const MAX_P1: $f = $u::MAX as $f;
                if *self > -1.0 && *self < MAX_P1 {
                    return Some(*self as $u);
                }
            }
            None
        }
    )*}
}

macro_rules! impl_to_primitive_float {
    ($T:ident) => (
        impl ToPrimitive for $T {
            impl_to_primitive_float_to_signed_int! { $T:
                fn to_isize -> isize;
                fn to_i8 -> i8;
                fn to_i16 -> i16;
                fn to_i32 -> i32;
                fn to_i64 -> i64;
                #[cfg(feature = "i128")]
                fn to_i128 -> i128;
            }

            impl_to_primitive_float_to_unsigned_int! { $T:
                fn to_usize -> usize;
                fn to_u8 -> u8;
                fn to_u16 -> u16;
                fn to_u32 -> u32;
                fn to_u64 -> u64;
                #[cfg(feature = "i128")]
                fn to_u128 -> u128;
            }

            impl_to_primitive_float_to_float! { $T:
                fn to_f32 -> f32;
                fn to_f64 -> f64;
            }
        }
    )
}

impl_to_primitive_float!(f32);
impl_to_primitive_float!(f64);

/// A generic trait for converting a number to a value.
pub trait FromPrimitive: Sized {
    /// Convert an `isize` to return an optional value of this type. If the
    /// value cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i8` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i16` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i64(n: i64) -> Option<Self>;

    /// Convert an `i128` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    ///
    /// This method is only available with feature `i128` enabled on Rust >= 1.26.
    ///
    /// The default implementation converts through `from_i64()`.  Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    #[cfg(feature = "i128")]
    fn from_i128(n: i128) -> Option<Self> {
        n.to_i64().and_then(FromPrimitive::from_i64)
    }

    /// Convert a `usize` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u8` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u16` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_u64(n: u64) -> Option<Self>;

    /// Convert an `u128` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    ///
    /// This method is only available with feature `i128` enabled on Rust >= 1.26.
    ///
    /// The default implementation converts through `from_u64()`.  Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    #[cfg(feature = "i128")]
    fn from_u128(n: u128) -> Option<Self> {
        n.to_u64().and_then(FromPrimitive::from_u64)
    }

    /// Convert a `f32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        FromPrimitive::from_f64(n as f64)
    }

    /// Convert a `f64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }
}

macro_rules! impl_from_primitive {
    ($T:ty, $to_ty:ident) => (
        #[allow(deprecated)]
        impl FromPrimitive for $T {
            #[inline] fn from_i8(n: i8) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_i16(n: i16) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_i32(n: i32) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_i64(n: i64) -> Option<$T> { n.$to_ty() }
            #[cfg(feature = "i128")]
            #[inline] fn from_i128(n: i128) -> Option<$T> { n.$to_ty() }

            #[inline] fn from_u8(n: u8) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_u16(n: u16) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_u32(n: u32) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_u64(n: u64) -> Option<$T> { n.$to_ty() }
            #[cfg(feature = "i128")]
            #[inline] fn from_u128(n: u128) -> Option<$T> { n.$to_ty() }

            #[inline] fn from_f32(n: f32) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_f64(n: f64) -> Option<$T> { n.$to_ty() }
        }
    )
}

impl_from_primitive!(isize, to_isize);
impl_from_primitive!(i8,    to_i8);
impl_from_primitive!(i16,   to_i16);
impl_from_primitive!(i32,   to_i32);
impl_from_primitive!(i64,   to_i64);
#[cfg(feature = "i128")]
impl_from_primitive!(i128,  to_i128);
impl_from_primitive!(usize, to_usize);
impl_from_primitive!(u8,    to_u8);
impl_from_primitive!(u16,   to_u16);
impl_from_primitive!(u32,   to_u32);
impl_from_primitive!(u64,   to_u64);
#[cfg(feature = "i128")]
impl_from_primitive!(u128,  to_u128);
impl_from_primitive!(f32,   to_f32);
impl_from_primitive!(f64,   to_f64);


macro_rules! impl_to_primitive_wrapping {
    ($( $(#[$cfg:meta])* fn $method:ident -> $i:ident ; )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(&self) -> Option<$i> {
            (self.0).$method()
        }
    )*}
}

impl<T: ToPrimitive> ToPrimitive for Wrapping<T> {
    impl_to_primitive_wrapping! {
        fn to_isize -> isize;
        fn to_i8 -> i8;
        fn to_i16 -> i16;
        fn to_i32 -> i32;
        fn to_i64 -> i64;
        #[cfg(feature = "i128")]
        fn to_i128 -> i128;

        fn to_usize -> usize;
        fn to_u8 -> u8;
        fn to_u16 -> u16;
        fn to_u32 -> u32;
        fn to_u64 -> u64;
        #[cfg(feature = "i128")]
        fn to_u128 -> u128;

        fn to_f32 -> f32;
        fn to_f64 -> f64;
    }
}

macro_rules! impl_from_primitive_wrapping {
    ($( $(#[$cfg:meta])* fn $method:ident ( $i:ident ); )*) => {$(
        #[inline]
        $(#[$cfg])*
        fn $method(n: $i) -> Option<Self> {
            T::$method(n).map(Wrapping)
        }
    )*}
}

impl<T: FromPrimitive> FromPrimitive for Wrapping<T> {
    impl_from_primitive_wrapping! {
        fn from_isize(isize);
        fn from_i8(i8);
        fn from_i16(i16);
        fn from_i32(i32);
        fn from_i64(i64);
        #[cfg(feature = "i128")]
        fn from_i128(i128);

        fn from_usize(usize);
        fn from_u8(u8);
        fn from_u16(u16);
        fn from_u32(u32);
        fn from_u64(u64);
        #[cfg(feature = "i128")]
        fn from_u128(u128);

        fn from_f32(f32);
        fn from_f64(f64);
    }
}


/// Cast from one machine scalar to another.
///
/// # Examples
///
/// ```
/// # use num_traits as num;
/// let twenty: f32 = num::cast(0x14).unwrap();
/// assert_eq!(twenty, 20f32);
/// ```
///
#[inline]
pub fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U> {
    NumCast::from(n)
}

/// An interface for casting between machine scalars.
pub trait NumCast: Sized + ToPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `ToPrimitive` trait.
    fn from<T: ToPrimitive>(n: T) -> Option<Self>;
}

macro_rules! impl_num_cast {
    ($T:ty, $conv:ident) => (
        impl NumCast for $T {
            #[inline]
            #[allow(deprecated)]
            fn from<N: ToPrimitive>(n: N) -> Option<$T> {
                // `$conv` could be generated using `concat_idents!`, but that
                // macro seems to be broken at the moment
                n.$conv()
            }
        }
    )
}

impl_num_cast!(u8,    to_u8);
impl_num_cast!(u16,   to_u16);
impl_num_cast!(u32,   to_u32);
impl_num_cast!(u64,   to_u64);
#[cfg(feature = "i128")]
impl_num_cast!(u128,  to_u128);
impl_num_cast!(usize, to_usize);
impl_num_cast!(i8,    to_i8);
impl_num_cast!(i16,   to_i16);
impl_num_cast!(i32,   to_i32);
impl_num_cast!(i64,   to_i64);
#[cfg(feature = "i128")]
impl_num_cast!(i128,  to_i128);
impl_num_cast!(isize, to_isize);
impl_num_cast!(f32,   to_f32);
impl_num_cast!(f64,   to_f64);

impl<T: NumCast> NumCast for Wrapping<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Wrapping)
    }
}

/// A generic interface for casting between machine scalars with the
/// `as` operator, which admits narrowing and precision loss.
/// Implementers of this trait AsPrimitive should behave like a primitive
/// numeric type (e.g. a newtype around another primitive), and the
/// intended conversion must never fail.
///
/// # Examples
///
/// ```
/// # use num_traits::AsPrimitive;
/// let three: i32 = (3.14159265f32).as_();
/// assert_eq!(three, 3);
/// ```
/// 
/// # Safety
/// 
/// Currently, some uses of the `as` operator are not entirely safe.
/// In particular, it is undefined behavior if:
/// 
/// - A truncated floating point value cannot fit in the target integer
///   type ([#10184](https://github.com/rust-lang/rust/issues/10184));
/// 
/// ```ignore
/// # use num_traits::AsPrimitive;
/// let x: u8 = (1.04E+17).as_(); // UB
/// ```
/// 
/// - Or a floating point value does not fit in another floating
///   point type ([#15536](https://github.com/rust-lang/rust/issues/15536)).
///
/// ```ignore
/// # use num_traits::AsPrimitive;
/// let x: f32 = (1e300f64).as_(); // UB
/// ```
/// 
pub trait AsPrimitive<T>: 'static + Copy
where
    T: 'static + Copy
{
    /// Convert a value to another, using the `as` operator.
    fn as_(self) -> T;
}

macro_rules! impl_as_primitive {
    (@ $T: ty => $(#[$cfg:meta])* impl $U: ty ) => {
        $(#[$cfg])*
        impl AsPrimitive<$U> for $T {
            #[inline] fn as_(self) -> $U { self as $U }
        }
    };
    (@ $T: ty => { $( $U: ty ),* } ) => {$(
        impl_as_primitive!(@ $T => impl $U);
    )*};
    ($T: ty => { $( $U: ty ),* } ) => {
        impl_as_primitive!(@ $T => { $( $U ),* });
        impl_as_primitive!(@ $T => { u8, u16, u32, u64, usize });
        impl_as_primitive!(@ $T => #[cfg(feature = "i128")] impl u128);
        impl_as_primitive!(@ $T => { i8, i16, i32, i64, isize });
        impl_as_primitive!(@ $T => #[cfg(feature = "i128")] impl i128);
    };
}

impl_as_primitive!(u8 => { char, f32, f64 });
impl_as_primitive!(i8 => { f32, f64 });
impl_as_primitive!(u16 => { f32, f64 });
impl_as_primitive!(i16 => { f32, f64 });
impl_as_primitive!(u32 => { f32, f64 });
impl_as_primitive!(i32 => { f32, f64 });
impl_as_primitive!(u64 => { f32, f64 });
impl_as_primitive!(i64 => { f32, f64 });
#[cfg(feature = "i128")]
impl_as_primitive!(u128 => { f32, f64 });
#[cfg(feature = "i128")]
impl_as_primitive!(i128 => { f32, f64 });
impl_as_primitive!(usize => { f32, f64 });
impl_as_primitive!(isize => { f32, f64 });
impl_as_primitive!(f32 => { f32, f64 });
impl_as_primitive!(f64 => { f32, f64 });
impl_as_primitive!(char => { char });
impl_as_primitive!(bool => {});

