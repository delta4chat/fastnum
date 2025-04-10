use crate::decimal::{
    dec::ControlBlock,
    Context,
    Notation, FormatStyle,
    RoundingMode,
    Signals, SignalsTraps,
};

const SIGNAL_TRAPS_SHIFT:    u8 = 27;
const FORMAT_NOTATION_SHIFT: u8 = 35;
const ROUNDING_MODE_SHIFT:   u8 = 37;

/// Control block (CB)
///
/// Memory layout:
///
/// |  Bit  |          Data           |         Bit Mask        |
/// |:-----:|:-----------------------:|:-----------------------:|
/// | `...` |      `...`              |          `...`          |
/// | 27    | T OP_CLAMPED            | `0x0000_0000_0800_0000` |
/// | 28    | T OP_DIV_BY_ZERO        | `0x0000_0000_1000_0000` |
/// | 29    | T OP_INVALID            | `0x0000_0000_2000_0000` |
/// | 30    | T OP_INEXACT            | `0x0000_0000_4000_0000` |
/// | 31    | T OP_OVERFLOW           | `0x0000_0000_8000_0000` |
/// | 32    | T OP_ROUNDED            | `0x0000_0001_0000_0000` |
/// | 33    | T OP_SUBNORMAL          | `0x0000_0002_0000_0000` |
/// | 34    | T OP_UNDERFLOW          | `0x0000_0004_0000_0000` |
/// | 35-36 | Format notation (2 bit) | `0x0000_0018_0000_0000` |
/// | 37-39 | Rounding mode   (3 bit) | `0x0000_00E0_0000_0000` |
/// | `...` |      `...`              |          `...`          |
impl ControlBlock {
    pub(super) const CONTEXT_MASK:         u64 = 0x0000_00FF_F800_0000;
    pub(super) const SIGNAL_TRAPS_MASK:    u64 = 0x0000_0007_F800_0000;
    pub(super) const FORMAT_NOTATION_MASK: u64 = 0x0000_0018_0000_0000;
    pub(super) const ROUNDING_MODE_MASK:   u64 = 0x0000_00E0_0000_0000;

    pub(super) const DEFAULT_CONTEXT: u64 = make_context(Context::default());

    #[inline(always)]
    pub const fn get_context(&self) -> Context {
        let mut format_style = FormatStyle::default();
        format_style.notation = self.get_format_notation();
        Context::new(self.get_rounding_mode(), self.get_signal_traps(), format_style)
    }

    #[inline(always)]
    pub const fn set_context(&mut self, ctx: Context) {
        self.0 = (self.0 & !Self::CONTEXT_MASK) | make_context(ctx);
    }

    #[inline(always)]
    pub const fn get_rounding_mode(&self) -> RoundingMode {
        let val = ((self.0 & Self::ROUNDING_MODE_MASK) >> ROUNDING_MODE_SHIFT) as u8;
        match RoundingMode::from_u8(val) {
            Some(rm) => rm,
            _ => {
                panic!("unexpected 3-bit integer is not valid value of RoundingMode");
            }
        }
    }

    #[inline(always)]
    pub const fn set_rounding_mode(&mut self, rm: RoundingMode) {
        self.0 = (self.0 & !Self::ROUNDING_MODE_MASK) | rounding_mode(rm);
    }

    #[inline(always)]
    pub const fn get_signal_traps(&self) -> SignalsTraps {
        SignalsTraps::new(Signals::new(
            ((self.0 & Self::SIGNAL_TRAPS_MASK) >> SIGNAL_TRAPS_SHIFT) as u8,
        ))
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub const fn set_signal_traps(&mut self, traps: SignalsTraps) {
        self.0 = (self.0 & !Self::SIGNAL_TRAPS_MASK) | signals_traps(traps);
    }

    #[inline(always)]
    pub const fn get_format_notation(&self) -> Notation {
        let val = ((self.0 & Self::FORMAT_NOTATION_MASK) >> FORMAT_NOTATION_SHIFT) as u8;
        match Notation::from_u8(val) {
            Some(n) => n,
            _ => {
                panic!("unexpected 2-bit integer is not valid value of Notation");
            }
        }
    }

    #[inline(always)]
    pub const fn set_format_notation(&mut self, n: Notation) {
        self.0 = (self.0 & !Self::FORMAT_NOTATION_MASK) | format_notation(n);
    }

    #[inline(always)]
    pub(super) const fn combine_ctx(&mut self, other: &Self) {
        self.0 |= other.0 & Self::SIGNAL_TRAPS_MASK;
        let rm = other.get_rounding_mode();
        if !rm.is_default() {
            self.set_rounding_mode(rm);
        }
        self.set_format_notation(other.get_format_notation());
    }
}

#[inline(always)]
const fn make_context(ctx: Context) -> u64 {
    rounding_mode(ctx.rounding_mode())
    |
    format_notation(ctx.notation())
    |
    signals_traps(ctx.signal_traps())
}

#[inline(always)]
const fn rounding_mode(rm: RoundingMode) -> u64 {
    (rm.to_u8() as u64) << ROUNDING_MODE_SHIFT
}

#[inline(always)]
const fn format_notation(n: Notation) -> u64 {
    (n.to_u8() as u64) << FORMAT_NOTATION_SHIFT
}

#[inline(always)]
const fn signals_traps(traps: SignalsTraps) -> u64 {
    (traps.signals().mask() as u64) << SIGNAL_TRAPS_SHIFT
}
