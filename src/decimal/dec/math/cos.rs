use crate::decimal::{
    dec::math::{consts::Consts, sin::sin, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn cos<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    let ox = *x;
    *x = Consts::FRAC_PI_2;
    x.sub_assign(&ox)
     .sin_assign()
}
