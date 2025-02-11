use crate::{
    decimal::{
        dec::{ControlBlock, ExtraPrecision},
        Decimal, Sign,
    },
    int::{math::ilog10, UInt},
};

type D<const N: usize> = Decimal<N>;

// @TODO Performance optimizations

#[inline]
const fn f2dec<const N: usize>(mant: u64, b_exp: i16, sign: Sign) -> D<N> {
    if b_exp == 0 {
        return D::new(UInt::from_digit(mant), ControlBlock::new(0, sign));
    }

    let uint = if b_exp < 0 {
        UInt::TWO.pow(-b_exp as u32)
    } else {
        UInt::TWO.pow(b_exp as u32)
    };

    let d_exp = ilog10(uint) as i16 + 1;
    let psi = D::<17>::new(uint, ControlBlock::new(d_exp, Sign::Plus));

    if b_exp < 0 {
        let d = D::new(UInt::from_digit(mant), ControlBlock::new(d_exp, sign));
        d.div(psi).transmute()
    } else {
        let d = D::new(UInt::from_digit(mant), ControlBlock::new(-d_exp, sign));
        d.mul(psi).transmute()
    }
}

macro_rules! from_float_impl {
    ($n: ident, $f: ident) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $f) -> D<N> {
            use crate::decimal::utils::types::$f::*;

            if is_nan(n) {
                return D::NAN;
            }

            let b = to_bits(n);

            let sign = if b & SIGN_MASK != 0 {
                Sign::Minus
            } else {
                Sign::Plus
            };

            let frac = b & MAN_MASK;
            let exp = b & EXP_MASK;

            if frac == 0 && exp == EXP_MASK {
                return D::INFINITY.with_cb(cb);
            }

            if frac == 0 && exp == 0 {
                return D::ZERO.with_cb(cb);
            }

            if exp == 0 {
                // subnormal

                let pow = (MAX_EXP - 2) as i16 + (MANTISSA_DIGITS - 1) as i16;
                f2dec(frac as u64, -pow, sign)
            } else {
                // normal

                let frac = frac | MAN_MASK_NORMAL;
                let pow = (exp >> (MANTISSA_DIGITS - 1)) as i16
                    - (MAX_EXP - 1) as i16
                    - (MANTISSA_DIGITS - 1) as i16;

                if pow == 0 {
                    Decimal::new(uint(frac), 0, sign)
                } else if pow < 0 {
                    let mut trailing_zeros = frac.trailing_zeros();
                    if trailing_zeros > -pow as u32 {
                        trailing_zeros = -pow as u32;
                    }

                    let reduced_frac = frac >> trailing_zeros;
                    let reduced_pow = pow + trailing_zeros as i16;

                    f2dec(reduced_frac as u64, reduced_pow, sign)
                } else {
                    f2dec(frac as u64, pow, sign)
                }
            }
        }
    };
}

from_float_impl!(from_f32, f32);
from_float_impl!(from_f64, f64);
