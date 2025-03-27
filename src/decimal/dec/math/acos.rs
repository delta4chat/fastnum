use crate::decimal::{
    dec::math::{asin::asin, consts::Consts, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn acos<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    asin(x);
    let ax = *x; // asin x
    *x = Consts::FRAC_PI_2;
    x.sub_assign(&ax)
}
