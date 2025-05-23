use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::{
    utils,
    dec::format,
    Decimal,
    Notation,
};

impl<const N: usize> Display for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Notation::*;
        match self.ctx.notation() {
            Unspecified => {
                format::format(
                    self.ctx,
                    self.digits.to_str_radix(10),
                    self.cb.get_scale(),
                    self.sign(),
                    f,
                )
            },
            Scientific => {
                self.write_scientific_notation(f)
            },
            FullScale => {
                format::format_full_scale(
                    self.digits.to_str_radix(10),
                    self.cb.get_scale(),
                    self.sign(),
                    f,
                )
            },
            Plain => {
                todo!("TODO to_plain_string");
            }
        }
    }
}

impl<const N: usize> LowerExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::format_exponential(
            self.digits.to_str_radix(10),
            self.cb.get_scale(),
            self.sign(),
            f,
            "e",
        )
    }
}

impl<const N: usize> UpperExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::format_exponential(
            self.digits.to_str_radix(10),
            self.cb.get_scale(),
            self.sign(),
            f,
            "E",
        )
    }
}

impl<const N: usize> Debug for Decimal<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        utils::fmt::debug_print(&self.digits, &self.cb, Self::type_name(), f)
    }
}
