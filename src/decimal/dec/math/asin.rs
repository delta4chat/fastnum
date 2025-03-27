use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::{add::add, consts::Consts, div::div, mul::mul, sqrt::sqrt, sub::sub},
        parse::from_u32,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn asin<const N: usize>(x: &mut D<N>) -> &mut D<N> {
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
        },
        Ordering::Equal => {
            *x = Consts::FRAC_PI_2;
            return x.neg_assign();
        },
        Ordering::Greater => {}
    }

    match x.cmp(&D::ONE) {
        Ordering::Less => {}
        Ordering::Equal => {
            *x = Consts::FRAC_PI_2;
            return x;
        }
        Ordering::Greater => {
            return x.signaling_nan();
        }
    }

    asin_reduction(x)
}

struct Reduction<const N: usize>;

impl<const N: usize> Reduction<N> {
    const K: D<N> = D::HALF; // TODO
}

#[inline]
const fn asin_reduction<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    debug_assert!(x.ge(&D::NEG_ONE));
    debug_assert!(x.le(&D::ONE));

    if x.abs().gt(&Reduction::K) {
        let x2 = x.mul(&x);

        let mut y = D::ONE;
        y.sub_assign(&x2);
         .sqrt_assign();
         .add_assign(&D::ONE);
         .sqrt_assign();
         .mul_assign(&Consts::SQRT_2);

        x.div_assign(&y);
        asin_reduction(x); // FIXME is this recursive can be avoided?
        x.mul_assign(&D::TWO)
    } else {
        taylor_series(x)
    }
}

#[inline]
const fn taylor_series<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    debug_assert!(x.ge(&D::ONE.neg()));
    debug_assert!(x.le(&D::ONE));

    let mut result = D::ZERO;
    let mut result_next;
    let mut item = x;

    let x2 = x.mul(&*x);
    //let x2 = mul(x, x);

    let mut i = 2;

    const MAX_ITER: u32 = Intrinsics::<N>::SERIES_MAX_ITERATIONS + 2;
    while i < MAX_ITER {
        result_next = result.add(&item);
        //add(result, item);

        if result.eq(&result_next) {
            break;
        }

        item.mul_assign(&x2)
            .mul_assign(&D::from_u32((2 * i - 3) * (2 * i - 3)))
            .div_assign(&D::from_u32((2 * i - 1) * (2 * i - 2)));

        result = result_next;
        i += 1;
    }

    let x_ctx = x.context();
    *x = result;
    x.set_ctx(x_ctx)
}
