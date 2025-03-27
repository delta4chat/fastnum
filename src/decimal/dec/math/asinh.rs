use crate::decimal::{
    dec::math::{add::add, ln::ln, mul::mul, sqrt::sqrt},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn asinh<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    let ox = *x; // original x
    x.mul_assign(&ox)
     .add_assign(&D::ONE)
     .sqrt_assign()
     .add_assign(&ox)
     .ln_assign()
}
