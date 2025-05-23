use crate::decimal::{Decimal, DecimalError, ParseError, UnsignedDecimal};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize> TryFrom<D<N>> for UD<N> {
    type Error = DecimalError;

    #[inline]
    fn try_from(d: D<N>) -> Result<Self, Self::Error> {
        if d.is_negative() {
            return Err(DecimalError::Invalid);
        }
        Ok(Self::new(d))
    }
}

macro_rules! from_impl {
    ($($name:ident $uint:ty,)*) => {
        impl<const N: usize> UD<N> {
            $(
                #[doc = concat!("Converts [`", stringify!($uint), "`] to [UnsignedDecimal].")]
                pub const fn $name(n: $uint) -> Self {
                    Self::new(D::$name(n))
                }
            )*
        }

        $(
            impl<const N: usize> From<$uint> for UD<N> {
                #[inline]
                fn from(n: $uint) -> Self {
                    Self::$name(n)
                }
            }
        )*
    }
}

macro_rules! try_from_impl {
    ($($name:ident $pname:ident $num:ty,)*) => {
        impl<const N: usize> UD<N> {
            $(
                #[inline]
                #[doc = concat!("Try converts [`", stringify!($num), "`] to [UnsignedDecimal].")]
                pub const fn $name(n: $num) -> Result<Self, ParseError> {
                    if n < 0 {
                        return Err(ParseError::Signed);
                    }
                    Ok(Self::new(D::$pname(n)))
                }
            )*
        }

        $(
            impl<const N: usize> TryFrom<$num> for UD<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $num) -> Result<Self, Self::Error> {
                    Self::$name(n)
                }
            }
        )*
    };
}

macro_rules! try_from_f_impl {
    ($($name:ident $pname:ident $num:ty,)*) => {
        impl<const N: usize> UD<N> {
            $(
                #[inline]
                #[doc = concat!("Try converts [`", stringify!($num), "`] to [UnsignedDecimal].")]
                pub const fn $name(n: $num) -> Result<Self, ParseError> {
                    if n.is_sign_negative() {
                        return Err(ParseError::Signed);
                    }
                    Ok(Self::new(D::$pname(n)))
                }
            )*
        }

        $(
            impl<const N: usize> TryFrom<$num> for UD<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $num) -> Result<Self, Self::Error> {
                    Self::$name(n)
                }
            }
        )*
    };
}

from_impl!(
    from_u8 u8,
    from_u16 u16,
    from_u32 u32,
    from_u64 u64,
    from_u128 u128,
    from_usize usize,
);

try_from_impl!(
    try_from_i8 from_i8 i8,
    try_from_i16 from_i16 i16,
    try_from_i32 from_i32 i32,
    try_from_i64 from_i64 i64,
    try_from_i128 from_i128 i128,
    try_from_isize from_isize isize,
);

try_from_f_impl!(
    try_from_f32 from_f32 f32,
    try_from_f64 from_f64 f64,
);
