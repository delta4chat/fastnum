//! Context of fastnum.

mod rounding_mode;
mod signal_traps;

pub use rounding_mode::RoundingMode;
pub use signal_traps::SignalsTraps;

use core::fmt::{Debug, Display, Formatter};

use crate::utils::{assert_eq_size, err_msg};

use crate::config::*;

/// the notation of formating.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Notation {
    /// Unspecified "default" notation.
    /// this is for compatibility older version of ControlBlock
    Unspecified = 0b00, // 0

    /// Scientific notation.
    Scientific = 0b01, // 1

    /// Full-scale notation (max precision, but uses scientific notation for fractional numbers).
    FullScale = 0b10, // 2

    /// Plain notation without scientific notation.
    /// (similar to `BigDecimal::to_plain_string`)
    Plain = 0b11, // 3
}

impl Notation {
    /// the default value of `Notation`.
    pub const DEFAULT: Self = Self::Unspecified;

    /// returns the default value of `Notation`.
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// convert u8 to Notation if it is 2-bit integer.
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            0b00 => Some(Self::Unspecified),
            0b01 => Some(Self::Scientific),
            0b10 => Some(Self::FullScale),
            0b11 => Some(Self::Plain),
            _ => None
        }
    }

    /// convert Notation to u8.
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Default for Notation {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Display for Notation {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Unspecified => "Unspecified Notation",
            Self::Scientific => "Scientific Notation",
            Self::FullScale => "Full Scale Notation",
            Self::Plain => "Plain Notation",
        })
    }
}
impl Debug for Notation {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self, f)
    }
}

/// format style.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct FormatStyle {
    /// the format notation.
    pub notation: Notation,

    /// maximum leading zeros allowed in full-scale format.
    /// e.g. it's 3 zeros for `0.0001` and `5.0002`.
    ///
    /// (this field will be ignored if `Notation` is `FullScale` or `Plain`)
    leading_zero_threshold: u32,

    /// how many trailing zeros are allowed in full-scale format.
    /// e.g. it's 3 zeros for `1.2000` and `5000`.
    ///
    /// (this field will be ignored if `Notation` is `FullScale` or `Plain`)
    trailing_zero_threshold: u32,
}

impl Default for FormatStyle {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl FormatStyle {
    /// the default value of `FormatStyle`.
    pub const DEFAULT: Self = Self {
        notation: Notation::DEFAULT,
        leading_zero_threshold: EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD,
        trailing_zero_threshold: EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD,
    };

    /// returns the default value of `FormatStyle`.
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// getter for `.leading_zero_threshold`
    pub const fn leading_zero_threshold(&self) -> u32 {
        self.leading_zero_threshold
    }

    /// getter for `.trailing_zero_threshold`
    pub const fn trailing_zero_threshold(&self) -> u32 {
        self.trailing_zero_threshold
    }

    /// setter for `.leading_zero_threshold`
    pub const fn set_leading_zero_threshold(&mut self, val: u32) -> &mut Self {
        if val == 0 {
            panic!(err_msg!("leading_zero_threshold should be great than zero!"));
        }
        self.leading_zero_threshold = val;
        self
    }

    /// setter for `.trailing_zero_threshold`
    pub const fn set_trailing_zero_threshold(&mut self, val: u32) -> &mut Self {
        self.trailing_zero_threshold = val;
        self
    }
}

impl Display for FormatStyle {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "FS={:?}", self.notation)?;
        if self.notation == Notation::default() {
            write!(f,
                   "(LeadingZeros:{},TrailingZeros:{})",
                   self.leading_zero_threshold,
                   self.trailing_zero_threshold
                  )?;
        }
        Ok(())
    }
}
impl Debug for FormatStyle {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self, f)
    }
}

/// # Decimal Context
///
/// The context represents the user-selectable parameters and rules which govern
/// the results of arithmetic operations (for example, the rounding mode when
/// rounding occurs).
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Context {
    rounding_mode: RoundingMode,
    signal_traps: SignalsTraps,
    format_style: FormatStyle,
}

impl Context {
    /// the default value of [Context].
    pub const DEFAULT: Self = Self {
        rounding_mode: RoundingMode::DEFAULT,
        signal_traps: SignalsTraps::DEFAULT,
        format_style: FormatStyle::DEFAULT,
    };

    /// Returns the [Default Decimal Context](#crate::default-decimal-context).
    #[inline(always)]
    #[must_use]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// set the given [RoundingMode] to the `Context`.
    #[inline(always)]
    pub const fn set_rounding_mode(&mut self, rm: RoundingMode) -> &mut Self {
        self.rounding_mode = rm;
        self
    }

    /// Apply the given [RoundingMode] to the `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.set_rounding_mode(rm);
        self
    }

    /// Clear traps to given `Context`.
    #[inline(always)]
    pub const fn clear_traps(&mut self) -> &mut Self {
        self.signal_traps = SignalsTraps::empty();
        self
    }

    /// Apply no traps to given `Context`.
    #[inline(always)]
    #[must_use]
    pub const fn without_traps(mut self) -> Self {
        self.clear_traps();
        self
    }

    /// Set the specified [SignalsTraps] to the given context.
    #[inline(always)]
    pub const fn set_signal_traps(&mut self, traps: SignalsTraps) -> &mut Self {
        self.signal_traps = traps;
        self
    }

    /// Method applies specified [SignalsTraps] to the given context.
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default().without_traps();
    ///
    /// // No panic! We can divide by zero!
    /// let res = dec256!(1.0).with_ctx(ctx) / dec256!(0).with_ctx(ctx);
    ///
    /// assert!(res.is_infinite());
    /// assert!(res.is_op_div_by_zero());
    /// assert!(res.is_op_invalid());
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn with_signal_traps(mut self, traps: SignalsTraps) -> Self {
        self.set_signal_traps(traps);
        self
    }

    /// Get [RoundingMode] of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    /// Get [SignalsTraps] of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn signal_traps(&self) -> SignalsTraps {
        self.signal_traps
    }

    /// Get [FormatStyle] of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn format_style<'a>(&'a self) -> &'a FormatStyle {
        &self.format_style
    }

    /// Get "notation" of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn notation(&self) -> Notation {
        self.format_style.notation
    }

    /// Set "notation" of given `Context`.
    #[inline(always)]
    pub const fn set_notation(&mut self, val: Notation) -> &mut Self {
        self.format_style.notation = val;
        self
    }

    /// Get "leading zero threshold" of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn leading_zero_threshold(&self) -> u32 {
        self.format_style.leading_zero_threshold
    }

    /// Get "trailing zero threshold" of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn trailing_zero_threshold(&self) -> u32 {
        self.format_style.trailing_zero_threshold
    }

    /// Set "leading zero threshold" of given `Context`.
    #[inline(always)]
    pub const fn set_leading_zero_threshold(&mut self, val: u32) -> &mut Self {
        self.format_style.set_leading_zero_threshold(val);
        self
    }

    /// Set "trailing zero threshold" of given `Context`.
    #[inline(always)]
    pub const fn set_trailing_zero_threshold(&mut self, val: u32) -> &mut Self {
        self.format_style.set_trailing_zero_threshold(val);
        self
    }

    #[inline(always)]
    pub(crate) const fn new(
        rounding_mode: RoundingMode,
        signal_traps: SignalsTraps,
        format_style: FormatStyle
    ) -> Self {
        Self {
            rounding_mode,
            signal_traps,
            format_style,
        }
    }

    #[inline(always)]
    pub(crate) const fn merge(mut self, other: Self) -> Self {
        self.signal_traps = self.signal_traps.merge(other.signal_traps);

        if !other.rounding_mode.is_default() {
            self.rounding_mode = other.rounding_mode;
        }

        self.format_style = other.format_style;

        self
    }
}

impl Display for Context {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "R={}, S={}, {}", self.rounding_mode, self.signal_traps, &self.format_style)
    }
}

impl Debug for Context {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self, f)
    }
}

assert_eq_size!(Context, u128);
