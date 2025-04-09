use core::fmt::{Debug, Display, Formatter};

use crate::utils::assert_eq_size;

use crate::config::*;

/// Determines how to calculate the last digit of the number
///
/// Default rounding mode is `HalfUp`.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundingMode {
    /// Special no-rounding mode.
    No = 0b000, // 0

    /// Always round away from zero
    ///
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 3.0
    /// * 1.6 → 2.0
    /// * 1.1 → 2.0
    /// * -1.1 → -2.0
    /// * -1.6 → -2.0
    /// * -2.5 → -3.0
    /// * -5.5 → -6.0
    Up = 0b001, // 1

    /// Always round towards zero
    ///
    /// * 5.5  →  5.0
    /// * 2.5  →  2.0
    /// * 1.6  →  1.0
    /// * 1.1  →  1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -1.0
    /// * -2.5 → -2.0
    /// * -5.5 → -5.0
    Down = 0b010, // 2

    /// Towards +∞
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 3.0
    /// * 1.6 → 2.0
    /// * 1.1 → 2.0
    /// * -1.1 → -1.0
    /// * -1.6 → -1.0
    /// * -2.5 → -2.0
    /// * -5.5 → -5.0
    Ceiling = 0b011, // 3

    /// Towards -∞
    ///
    /// * 5.5 → 5.0
    /// * 2.5 → 2.0
    /// * 1.6 → 1.0
    /// * 1.1 → 1.0
    /// * -1.1 → -2.0
    /// * -1.6 → -2.0
    /// * -2.5 → -3.0
    /// * -5.5 → -6.0
    Floor = 0b100, // 4

    /// Round to 'nearest neighbor', or up if ending decimal is 5
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 3.0
    /// * 1.6 → 2.0
    /// * 1.1 → 1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -2.0
    /// * -2.5 → -3.0
    /// * -5.5 → -6.0
    HalfUp = 0b101, // 5

    /// Round to 'nearest neighbor', or down if ending decimal is 5
    ///
    /// * 5.5 → 5.0
    /// * 2.5 → 2.0
    /// * 1.6 → 2.0
    /// * 1.1 → 1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -2.0
    /// * -2.5 → -2.0
    /// * -5.5 → -5.0
    HalfDown = 0b110, // 6

    /// Round to 'nearest neighbor', if equidistant, round towards nearest even digit
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 2.0
    /// * 1.6 → 2.0
    /// * 1.1 → 1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -2.0
    /// * -2.5 → -2.0
    /// * -5.5 → -6.0
    HalfEven = 0b111, // 7
}

impl Default for RoundingMode {
    #[inline(always)]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl RoundingMode {
    /// default value of `RoundingMode`.
    pub const DEFAULT: Self = DEFAULT_ROUNDING_MODE;

    /// Returns default rounding mode.
    #[inline(always)]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// convert u8 to RoundingMode if it is 3-bit integer.
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            0b000 => Some(Self::No),
            0b001 => Some(Self::Up),
            0b010 => Some(Self::Down),
            0b011 => Some(Self::Ceiling),
            0b100 => Some(Self::Floor),
            0b101 => Some(Self::HalfUp),
            0b110 => Some(Self::HalfDown),
            0b111 => Some(Self::HalfEven),
            _ => None
        }
    }

    /// convert RoundingMode to u8.
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Returns `true` if given [RoundingMode] is
    /// [default](crate#rounding-mode).
    #[inline(always)]
    pub const fn is_default(&self) -> bool {
        self.to_u8() == Self::DEFAULT.to_u8()
    }
}

impl Display for RoundingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let rm = match self {
            RoundingMode::No => "No",
            RoundingMode::Up => "Up",
            RoundingMode::Down => "Down",
            RoundingMode::Ceiling => "Ceiling",
            RoundingMode::Floor => "Floor",
            RoundingMode::HalfUp => "HalfUp",
            RoundingMode::HalfDown => "HalfDown",
            RoundingMode::HalfEven => "HalfEven",
        };
        f.write_str(rm)
    }
}

impl Debug for RoundingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

assert_eq_size!(RoundingMode, u8);
