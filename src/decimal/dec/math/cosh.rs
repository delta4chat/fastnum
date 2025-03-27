use crate::decimal::{
    dec::math::{add::add, div::div, exp::exp, mul::mul},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn cosh<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    let e = x.exp();
    *x = D::ONE;
    x.div_assign(&e)
     .add_assign(&e)
     .mul(&D::HALF)
}
