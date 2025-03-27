use crate::{
    decimal::{
        dec::{
            construct::construct,
            intrinsics::Intrinsics,
            math::{add::add, mul::mul, sub::sub, utils::correct},
            ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal,
    },
    int::{
        math::{div_rem, div_rem_digit},
        UInt,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

#[inline]
pub(crate) const fn div<const N: usize>(dividend: &mut D<N>, divisor: &D<N>) -> &mut D<N> {
    if dividend.is_nan() {
        return dividend.compound(divisor).op_invalid();
    }

    if divisor.is_nan() {
        let mut result = *divisor;
        result.compound(&dividend).op_invalid();
        *dividend = result;
        return dividend;
    }

    let sign = dividend.sign().div(divisor.sign());
    let mut signals = Signals::combine(dividend.cb.get_signals(), divisor.cb.get_signals());
    let ctx = Context::merge(dividend.cb.get_context(), divisor.cb.get_context());

    if dividend.is_infinite() && divisor.is_infinite() {
        *dividend = D::SIGNALING_NAN;
        return dividend.raise_signals(signals).set_ctx(ctx);
    }

    if divisor.is_zero() {
        *dividend = D::INFINITY;

        dividend
            .raise_signals(signals)
            .set_ctx(ctx)
            .set_sign(sign)
            .raise_signals(Signals::OP_DIV_BY_ZERO)
            .op_invalid()
    } else if dividend.is_zero() || divisor.is_one() {
        dividend.compound(divisor).set_sign(sign)
    } else if dividend.is_infinite() {
        *dividend = D::INFINITY;

        dividend.raise_signals(signals)
                .set_ctx(ctx)
                .set_sign(sign)
    } else if divisor.is_infinite() {
        *dividend = D::ZERO;

        dividend.raise_signals(signals)
                .set_ctx(ctx)
                .set_sign(sign)
    } else {
        let mut divisor = *divisor;
        let correction = div_correction(&mut dividend, &mut divisor);

        let mut exp = dividend.cb.get_exponent() - divisor.cb.get_exponent();
        let (mut digits, mut remainder) = div_rem(dividend.digits, divisor.digits);

        if ! remainder.is_zero() {
            let mut quotient;
            let mut digits_prev;

            while ! remainder.is_zero() {
                (quotient, remainder) = div_rem_next(remainder, divisor.digits);

                if digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
                    let extra_digits = extra_precision(quotient, remainder, divisor.digits);

                    signals.raise(Signals::OP_INEXACT);
                    signals.raise(Signals::OP_ROUNDED);

                    let result = construct(digits, exp, sign, signals, ctx, extra_digits);
                    return correct(result, correction);
                }

                digits_prev = digits.strict_mul(UInt::TEN);
                exp -= 1;

                if digits_prev.gt(&UInt::MAX.strict_sub(quotient)) {
                    let extra_digits = extra_precision(quotient, remainder, divisor.digits);

                    signals.raise(Signals::OP_INEXACT);
                    signals.raise(Signals::OP_ROUNDED);

                    let result = construct(UInt::MAX, exp, sign, signals, ctx, extra_digits);
                    return correct(result, correction);
                }

                digits = digits_prev.strict_add(quotient);
            }
        }

        let result = construct(digits, exp, sign, signals, ctx, ExtraPrecision::new());
        correct(result, correction)
    }
}

#[inline]
const fn extra_precision<const N: usize>(
    mut quotient: U<N>,
    mut remainder: U<N>,
    divisor: U<N>,
) -> ExtraPrecision {
    let mut ep = extra_digits(quotient);

    while !remainder.is_zero() && ep.count() < ExtraPrecision::EXTRA_PRECISION_DIGITS {
        (quotient, remainder) = div_rem_next(remainder, divisor);
        let ep_next = extra_digits(quotient);

        ep.push_back(ep_next);
    }

    ep
}

#[inline]
const fn extra_digits<const N: usize>(mut quotient: U<N>) -> ExtraPrecision {
    let mut ep = ExtraPrecision::new();
    let mut digit;

    (quotient, digit) = div_rem_digit(quotient, 10);
    ep.push_digit(digit);

    while !quotient.is_zero() {
        (quotient, digit) = div_rem_digit(quotient, 10);
        ep.push_digit(digit);
    }

    ep
}

#[inline]
const fn div_rem_next<const N: usize>(mut remainder: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    if remainder.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
        mul_div_rem_wide(remainder, UInt::TEN, divisor)
    } else {
        remainder = remainder.strict_mul(UInt::TEN);
        div_rem(remainder, divisor)
    }
}

#[inline]
const fn mul_div_rem_wide<const N: usize>(lhs: U<N>, rhs: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let (low, high) = lhs.widening_mul(rhs);

    if high.is_zero() {
        div_rem(low, divisor)
    } else {
        div_rem_wide(low, high, divisor)
    }
}

#[inline]
const fn overflow_remainder<const N: usize>(
    mut quotient: U<N>,
    mut remainder: U<N>,
    add: U<N>,
    divisor: U<N>,
) -> (U<N>, U<N>) {
    let overflow;

    (remainder, overflow) = remainder.overflowing_add(add);

    if overflow {
        quotient = quotient.strict_add(UInt::ONE);
        (quotient, remainder) =
            overflow_remainder(quotient, remainder, UInt::MAX.strict_sub(divisor).strict_add(UInt::ONE), divisor);
    }

    let q;
    (q, remainder) = div_rem(remainder, divisor);
    quotient = quotient.strict_add(q);

    (quotient, remainder)
}

#[inline]
const fn div_rem_wide<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let mut quotient;
    let mut remainder;

    (quotient, remainder) = div_rem(low, divisor);

    let (high_quotient, high_remainder) =
        mul_div_rem_wide(UInt::MAX.shr(1).add(UInt::ONE), high, divisor);

    (quotient, remainder) = overflow_remainder(quotient, remainder, high_remainder, divisor);
    (quotient, remainder) = overflow_remainder(quotient, remainder, high_remainder, divisor);

    quotient = quotient.strict_add(high_quotient).strict_add(high_quotient);

    (quotient, remainder)
}

#[inline]
const fn div_correction<const N: usize>(dividend: &mut D<N>, divisor: &mut D<N>) -> D<N> {
    let xi_dividend = dividend.cb.take_extra_precision_decimal();
    let xi_divisor = divisor.cb.take_extra_precision_decimal();

    if xi_dividend.is_zero() && xi_divisor.is_zero() {
        D::ZERO
    } else if xi_divisor.is_zero() {
        let x = xi_dividend.div(&divisor);

        if x.is_op_underflow() {
            D::ZERO
        } else {
            x
        }
    } else {
        let mut x;
        if xi_dividend.is_zero() {
            x = dividend.mul(&xi_divisor);
            x.neg_assign().without_extra_digits();
        } else {
            x = divisor.mul(xi_dividend);
            x.sub_assign(&(dividend.mul(&xi_divisor)))
             .without_extra_digits();
        };

        if x.is_zero() || x.is_op_underflow() {
            D::ZERO
        } else {
            let mut y = divisor.mul(&divisor);
            y.add_assign(&(divisor.mul(&xi_divisor))).without_extra_digits();

            if z.is_op_underflow() {
                D::ZERO
            } else {
                x.div_assign(&y);
                x
            }
        }
    }
}
