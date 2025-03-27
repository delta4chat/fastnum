use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        math::sub::sub_abs,
        scale::{extend_scale_to, rescale},
        utils,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn add<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    if lhs.is_nan() {
        return lhs.compound(rhs).op_invalid();
    }

    if rhs.is_nan() {
        let mut rhs = *rhs;
        rhs.compound(&*lhs).op_invalid();
        *lhs = rhs;
        return lhs;
    }

    match (lhs.cb.is_negative(), rhs.cb.is_negative()) {
        (false, false) => add_abs(lhs, rhs),
        (true, true) => add_abs(lhs.neg_assign(), &(rhs.neg())).neg(),
        (false, true) => sub_abs(lhs, &(rhs.neg())),
        (true, false) => sub_abs(lhs.neg_assign(), rhs),
    }
}

#[inline]
pub(crate) const fn add_abs<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() {
        return lhs.compound(rhs);
    }

    if rhs.is_infinite() {
        let mut rhs = *rhs;
        rhs.compound(&lhs);
        *lhs = rhs;
        return lhs;
    }

    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.cb.get_scale()).compound(rhs);
    }

    if lhs.is_zero() {
        let mut rhs = *rhs;
        extend_scale_to(&mut rhs, lhs.cb.get_scale()).compound(&lhs);
        *lhs = rhs;
        return lhs;
    }

    match lhs.cb.scale_cmp(&rhs.cb) {
        Ordering::Less => add_rescale(lhs, rhs),
        Ordering::Equal => add_aligned(lhs, rhs),
        Ordering::Greater => add_rescale(lhs, rhs),
    }
}

#[inline]
const fn add_rescale<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    rescale(lhs, rhs.cb.get_scale());

    if lhs.is_op_clamped() {
        let mut rhs = *rhs;
        rescale(&mut rhs, lhs.cb.get_scale());
        add_aligned(lhs, &rhs)
    } else {
        add_aligned(lhs, rhs)
    }
}

#[inline]
const fn add_aligned<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    debug_assert!(lhs.cb.get_scale() == rhs.cb.get_scale());

    let (digits, overflow) = lhs.digits.overflowing_add(rhs.digits);

    if ! overflow {
        lhs.digits = digits;
        utils::add_extra_precision(lhs, rhs);
        lhs.compound(rhs)
    } else if let (scale, false) = lhs.cb.get_scale().overflowing_sub(1) {
        let mut rhs = *rhs;

        rescale(lhs, scale);
        rescale(&mut rhs, scale);

        add_aligned(lhs, &rhs)
    } else {
        lhs.compound(rhs).op_overflow()
    }
}
