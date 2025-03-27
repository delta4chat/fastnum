use crate::decimal::{
    dec::math::{add::add, div::div, ln::ln, mul::mul, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atanh<const N: usize>(x: &mut D<N>) -> &mut D<N> {
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

    if x.abs().gt(&D::ONE) {
        return x.signaling_nan();
    }

    let mut y = D::ONE;
    y.sub_assign(&x);

    x.add_assign(&D::ONE)
     .div_assign(&y)
     .ln_assign()
     .mul(&D::HALF)
}
