use core::cmp::Ordering;

use crate::{
    decimal::{
        dec::{
            math::add::add_abs,
            scale::{extend_scale_to, rescale},
            utils,
        },
        Decimal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sub<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    if lhs.is_nan() {
        lhs.compound(rhs).op_invalid();
        return lhs;
    }

    if rhs.is_nan() {
        let mut rhs = *rhs;
        rhs.compound(&*lhs).op_invalid();
        *lhs = *rhs;
        return lhs;
    }

    match (lhs.cb.is_negative(), rhs.cb.is_negative()) {
        (false, false) => sub_abs(lhs, rhs),
        (true, true) => sub_abs(rhs.neg(), lhs.neg()),
        (false, true) => add_abs(lhs, rhs.neg()),
        (true, false) => add_abs(lhs.neg(), rhs).neg(),
    }
}

#[inline]
pub(crate) const fn sub_abs<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() && rhs.is_infinite() {
        let lhs_ctx = lhs.context();
        *lhs = D::SIGNALING_NAN;
        return lhs.set_ctx(lhs_ctx).compound(rhs);
    } else if lhs.is_infinite() {
        return lhs.compound(rhs);
    } else if rhs.is_infinite() {
        let mut rhs = *rhs;
        rhs.compound(&*lhs).neg();
        *lhs = *rhs;
        return lhs;
    }

    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.cb.get_scale()).compound(rhs);
    }

    if lhs.is_zero() {
        let mut rhs = *rhs;
        extend_scale_to(rhs, lhs.cb.get_scale())
            .compound(lhs)
            .neg();
        *lhs = rhs;
        return lhs;
    }

    match lhs.cb.scale_cmp(&rhs.cb) {
        Ordering::Equal => {},
        Ordering::Less => {
            rescale(lhs, rhs.cb.get_scale());
            if lhs.is_op_clamped() {
                rescale(rhs, lhs.cb.get_scale());
            }
        },
        Ordering::Greater => {
            rescale(rhs, lhs.cb.get_scale());
            if rhs.is_op_clamped() {
                rescale(lhs, rhs.cb.get_scale());
            }
        }
    }

    sub_aligned(lhs, rhs)
}

#[inline]
const fn sub_aligned<const N: usize>(lhs: &mut D<N>, rhs: &D<N>) -> &mut D<N> {
    debug_assert!(lhs.cb.get_scale() == rhs.cb.get_scale());

    match lhs.digits.cmp(&rhs.digits) {
        Ordering::Less => {
            let mut rhs = *rhs;
            rhs.digits = rhs.digits.strict_sub(lhs.digits);
            utils::sub_extra_precision(&mut rhs, &*lhs);
            rhs.compound(&lhs).neg();
            *lhs = rhs;
            lhs
        }
        Ordering::Equal => {
            lhs.digits = UInt::ZERO;
            utils::sub_extra_precision(lhs, rhs);
            lhs.compound(rhs)
        }
        Ordering::Greater => {
            lhs.digits = lhs.digits.strict_sub(rhs.digits);
            utils::sub_extra_precision(lhs, rhs);
            lhs.compound(rhs)
        }
    }
}
