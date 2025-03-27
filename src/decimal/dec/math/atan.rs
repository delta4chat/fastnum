use core::cmp::Ordering;

use crate::decimal::{
    dec::math::{add::add, asin::asin, consts::Consts, div::div, mul::mul, sqrt::sqrt},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atan<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        let ctx = x.context();
        *x = D::ZERO;
        return x.set_ctx(ctx);
    }

    if x.is_infinite() {
        return x.signaling_nan();
    }

    match x.cmp(&D::NEG_ONE) {
        Ordering::Less => {
            return x.signaling_nan();
        }
        Ordering::Equal => {
            *x = Consts::FRAC_PI_4;
            return x.neg();
        }
        Ordering::Greater => {}
    }

    match x.cmp(&D::ONE) {
        Ordering::Less => {}
        Ordering::Equal => {
            *x = Consts::FRAC_PI_4;
            return x;
        }
        Ordering::Greater => {
            return x.signaling_nan();
        }
    }

    let mut y = *x;
    y.mul_assign(&x)
     .add_assign(&D::ONE)
     .sqrt_assign();

    x.div_assign(&y)
     .asin_assign()
}
