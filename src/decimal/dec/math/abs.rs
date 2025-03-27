use crate::decimal::Decimal;

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn abs<const N: usize>(d: &mut D<N>) -> &mut D<N> {
    if d.is_nan() {
        d.signaling_nan();
    } else {
        d.cb.abs();
    }
    d
}
