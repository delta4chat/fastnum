use crate::decimal::{dec::math::nth_root::nth_root, Decimal};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn cbrt<const N: usize>(x: &mut D<N>) -> &mut D<N> {
    nth_root(x, 3)
}
