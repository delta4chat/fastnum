mod float_const;

use num_traits::{ConstOne, ConstZero, FromPrimitive, Num, One, ToPrimitive, Zero};

use crate::decimal::{Context, ParseError, UnsignedDecimal};

impl<const N: usize> One for UnsignedDecimal<N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl<const N: usize> ConstOne for UnsignedDecimal<N> {
    const ONE: Self = Self::ONE;
}

impl<const N: usize> Zero for UnsignedDecimal<N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl<const N: usize> ConstZero for UnsignedDecimal<N> {
    const ZERO: Self = Self::ZERO;
}

impl<const N: usize> Num for UnsignedDecimal<N> {
    type FromStrRadixErr = ParseError;

    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(ParseError::InvalidRadix);
        }
        Self::from_str(str, Context::default())
    }
}

impl<const N: usize> ToPrimitive for UnsignedDecimal<N> {
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize().ok()
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8().ok()
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16().ok()
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32().ok()
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64().ok()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128().ok()
    }

    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize().ok()
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8().ok()
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16().ok()
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32().ok()
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64().ok()
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128().ok()
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some(self.0.to_f32())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.0.to_f64())
    }
}

impl<const N: usize> FromPrimitive for UnsignedDecimal<N> {
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        Self::try_from(n).ok()
    }
}
