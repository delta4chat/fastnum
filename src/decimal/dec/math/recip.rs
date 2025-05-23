use crate::decimal::{
    dec::{
        convert::to_f64,
        intrinsics::Intrinsics,
        math::{add::add, mul::mul, sub::sub},
        parse::from_f64,
        scale,
        scale::extend_scale_to,
    },
    Decimal, Signals,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn recip<const N: usize>(d: D<N>) -> D<N> {
    if d.is_nan() {
        return d.op_invalid();
    }

    if d.is_zero() {
        return D::INFINITY
            .set_ctx(d.context())
            .compound(&d)
            .raise_signals(Signals::OP_DIV_BY_ZERO);
    }

    if d.is_infinite() {
        return D::ZERO.set_ctx(d.context()).compound(&d);
    }

    let scale = d.cb.get_scale();

    let approx_f64 = to_f64(d);
    let approx_result = 1.0 / approx_f64;

    let mut result = from_f64(approx_result).compound(&d);

    let mut result_next;

    let mut i = 1;

    while result.is_ok() && i < Intrinsics::<N>::SERIES_MAX_ITERATIONS {
        result_next = add(result, mul(result, sub(D::ONE, mul(result, d))));

        if result.eq(&result_next) {
            break;
        }

        result = result_next;
        i += 1;
    }

    extend_scale_to(scale::reduce(result), scale).raise_signals(
        Signals::OP_INEXACT
            .combine(Signals::OP_CLAMPED)
            .combine(Signals::OP_ROUNDED),
    )
}
