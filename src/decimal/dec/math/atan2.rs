use core::cmp::Ordering::*;

use crate::decimal::{
    dec::{
        cmp::cmp,
        math::{add::add, atan::atan, div::div, sub::sub},
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atan2<const N: usize>(y: &mut D<N>, x: &D<N>) -> &mut D<N> {
    match (x.cmp(&D::ZERO), y.cmp(&D::ZERO)) {
        (Equal, Equal) => x.compound(&y).signaling_nan(),
        (Greater, _) => atan(div(y, x)),
        (Less, Greater | Equal) => add(atan(div(y, x)), &D::PI),
        (Less, Less) => sub(atan(div(y, x)), &D::PI),
        (Equal, Greater) => D::FRAC_PI_2,
        (Equal, Less) => D::FRAC_PI_2.neg(),
    }
}
