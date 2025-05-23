mod float;
mod float_const;
mod float_core;
mod from_primitive;
mod to_primitive;

use num_traits::{ConstOne, ConstZero, Num, NumCast, One, Signed, ToPrimitive, Zero};

use crate::decimal::{Context, Decimal, ParseError};

impl<const N: usize> One for Decimal<N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl<const N: usize> ConstOne for Decimal<N> {
    const ONE: Self = Self::ONE;
}

impl<const N: usize> Zero for Decimal<N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl<const N: usize> ConstZero for Decimal<N> {
    const ZERO: Self = Self::ZERO;
}

impl<const N: usize> Num for Decimal<N> {
    type FromStrRadixErr = ParseError;

    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(ParseError::InvalidRadix);
        }
        Self::from_str(str, Context::default())
    }
}

impl<const N: usize> Signed for Decimal<N> {
    #[inline]
    fn abs(&self) -> Self {
        (*self).abs()
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if self.le(other) {
            Self::ZERO
        } else {
            *self - *other
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        self.signum()
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.is_negative()
    }
}

impl<const N: usize> NumCast for Decimal<N> {
    #[inline]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        T::to_f64(&n).map(Self::from_f64)
    }
}
