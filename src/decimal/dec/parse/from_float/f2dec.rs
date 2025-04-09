mod psi_map;

use crate::{
    decimal::{
        dec::{
            math::{div::div, mul::mul},
            ControlBlock, ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal, Sign,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn f2dec<const N: usize>(mant: u64, b_exp: i16, sign: Sign) -> D<N> {
    if b_exp == 0 {
        return D::new(
            UInt::from_digit(mant),
            ControlBlock::new(
                0,
                sign,
                Signals::EMPTY,
                Context::DEFAULT,
                ExtraPrecision::new(),
            ),
            Context::DEFAULT,
        );
    }

    if b_exp < 0 {
        let (d_exp, psi) = psi_map::lookup(-b_exp);

        let d = D::new(
            UInt::from_digit(mant),
            ControlBlock::new(
                d_exp,
                sign,
                Signals::EMPTY,
                Context::DEFAULT,
                ExtraPrecision::new(),
            ),
            Context::DEFAULT,
        );

        div(d, psi).round_extra_precision().check()
    } else {
        let (d_exp, psi) = psi_map::lookup(b_exp);

        let d = D::new(
            UInt::from_digit(mant),
            ControlBlock::new(
                -d_exp,
                sign,
                Signals::EMPTY,
                Context::DEFAULT,
                ExtraPrecision::new(),
            ),
            Context::DEFAULT,
        );

        mul(d, psi).round_extra_precision().check()
    }
}
