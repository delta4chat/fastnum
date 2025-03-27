use crate::decimal::{
    dec::math::{add::add, ln::ln, mul::mul, sqrt::sqrt, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn acosh<const N: usize>(x: &mut D<N>) -> &mut D<N> {
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

    if x.le(&D::ONE) {
        return x.signaling_nan();
    }

    let ox = *x; // original x
    x.mul_assign(&ox)
     .sub_assign(&D::ONE)
     .sqrt_assign()
     .add_assign(&ox)
     .ln_assign()
}
