//! # Big Integers
//!
//! Under the hood [bnum](https://docs.rs/bnum/latest/bnum/) is currently used as the backend as most meeting the
//! requirements.
//! Subsequently, the implementation can be replaced in favor of its own
//! implementation, which enables `SIMD`.

#[cfg(debug_assertions)]
mod assertions;

mod doc;
mod error;
mod uint;

use doc::int_type_doc;

#[doc(hidden)]
pub mod parse;

pub(crate) use uint::*;

pub use error::ParseError;

/// Big unsigned integer generic type, of fixed size which must be known at compile time.
pub type UInt<const N: usize> = bnum::BUint<N>;

/// Big signed integer generic type, of fixed size which must be known at compile time.
pub type Int<const N: usize> = bnum::BInt<N>;

macro_rules! _macro_impl {
    ($bits:literal, $sign:ident, $type_name:ident, $macro_name:ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct [`", stringify!($type_name), "`](crate::", stringify!($type_name), ") (", $bits, "-bit ", stringify!($sign), " integer) from literals.")]
        ///
        ///
        /// # Examples:
        ///
        /// ```
        #[doc = concat!("use fastnum::{", stringify!($type_name), ", ", stringify!($macro_name), "};")]
        ///
        #[doc = concat!("const N: ", stringify!($type_name), " = ", stringify!($macro_name), "!(100);")]
        #[doc = concat!("let x = ", stringify!($macro_name), "!(1);")]
        #[doc = concat!("assert!(", stringify!($macro_name), "!(0).is_zero());")]
        /// println!("{x}");
        /// ```
        ///
        macro_rules! $macro_name {
            ($lit:expr) => {{
                const __INT: $int = $crate::int::parse::$sign::parse_str(stringify!($lit));
                __INT
            }};
        }
    }
}

macro_rules! int_types_impl {
    ($($bits:literal, $uint:ident/$uint_macro:ident, $int:ident/$int_macro:ident;)*) => {
        $(
            #[doc = int_type_doc!($bits, "unsigned")]
            pub type $uint = UInt::<{$bits / 64}>;

            #[doc = int_type_doc!($bits, "signed")]
            pub type $int = Int::<{$bits / 64}>;

            _macro_impl!($bits, unsigned, $uint, $uint_macro);
            _macro_impl!($bits,   signed, $int,  $int_macro);
        )*
    };
}

int_types_impl!(
    64,   U64/u64,     I64/i64;
    128,  U128/u128,   I128/i128;
    256,  U256/u256,   I256/i256;
    512,  U512/u512,   I512/i512;
    1024, U1024/u1024, I1024/i1024;
    2048, U2048/u2048, I2048/i2048;
    4096, U4096/u4096, I4096/i4096;
    8192, U8192/u8192, I8192/i8192;
);

#[cfg(feature="more-types")]
int_types_impl!(
    192,   U192/u192,     I192/i192;
    320,   U320/u320,     I320/i320;
    384,   U384/u384,     I384/i384;
    448,   U448/u448,     I448/i448;
    576,   U576/u576,     I576/i576;
    640,   U640/u640,     I640/i640;
    704,   U704/u704,     I704/i704;
    768,   U768/u768,     I768/i768;
    832,   U832/u832,     I832/i832;
    896,   U896/u896,     I896/i896;
    960,   U960/u960,     I960/i960;
    1088,  U1088/u1088,   I1088/i1088;
    1152,  U1152/u1152,   I1152/i1152;
    1216,  U1216/u1216,   I1216/i1216;
    1280,  U1280/u1280,   I1280/i1280;
    1344,  U1344/u1344,   I1344/i1344;
    1408,  U1408/u1408,   I1408/i1408;
    1472,  U1472/u1472,   I1472/i1472;
    1536,  U1536/u1536,   I1536/i1536;
    1600,  U1600/u1600,   I1600/i1600;
    1664,  U1664/u1664,   I1664/i1664;
    1728,  U1728/u1728,   I1728/i1728;
    1792,  U1792/u1792,   I1792/i1792;
    1856,  U1856/u1856,   I1856/i1856;
    1920,  U1920/u1920,   I1920/i1920;
    1984,  U1984/u1984,   I1984/i1984;
    2112,  U2112/u2112,   I2112/i2112;
    2176,  U2176/u2176,   I2176/i2176;
    2240,  U2240/u2240,   I2240/i2240;
    2304,  U2304/u2304,   I2304/i2304;
    2368,  U2368/u2368,   I2368/i2368;
    2432,  U2432/u2432,   I2432/i2432;
    2496,  U2496/u2496,   I2496/i2496;
    2560,  U2560/u2560,   I2560/i2560;
    2624,  U2624/u2624,   I2624/i2624;
    2688,  U2688/u2688,   I2688/i2688;
    2752,  U2752/u2752,   I2752/i2752;
    2816,  U2816/u2816,   I2816/i2816;
    2880,  U2880/u2880,   I2880/i2880;
    2944,  U2944/u2944,   I2944/i2944;
    3008,  U3008/u3008,   I3008/i3008;
    3072,  U3072/u3072,   I3072/i3072;
    3136,  U3136/u3136,   I3136/i3136;
    3200,  U3200/u3200,   I3200/i3200;
    3264,  U3264/u3264,   I3264/i3264;
    3328,  U3328/u3328,   I3328/i3328;
    3392,  U3392/u3392,   I3392/i3392;
    3456,  U3456/u3456,   I3456/i3456;
    3520,  U3520/u3520,   I3520/i3520;
    3584,  U3584/u3584,   I3584/i3584;
    3648,  U3648/u3648,   I3648/i3648;
    3712,  U3712/u3712,   I3712/i3712;
    3776,  U3776/u3776,   I3776/i3776;
    3840,  U3840/u3840,   I3840/i3840;
    3904,  U3904/u3904,   I3904/i3904;
    3968,  U3968/u3968,   I3968/i3968;
    4032,  U4032/u4032,   I4032/i4032;
    4160,  U4160/u4160,   I4160/i4160;
    4224,  U4224/u4224,   I4224/i4224;
    4288,  U4288/u4288,   I4288/i4288;
    4352,  U4352/u4352,   I4352/i4352;
    4416,  U4416/u4416,   I4416/i4416;
    4480,  U4480/u4480,   I4480/i4480;
    4544,  U4544/u4544,   I4544/i4544;
    4608,  U4608/u4608,   I4608/i4608;
    4672,  U4672/u4672,   I4672/i4672;
    4736,  U4736/u4736,   I4736/i4736;
    4800,  U4800/u4800,   I4800/i4800;
    4864,  U4864/u4864,   I4864/i4864;
    4928,  U4928/u4928,   I4928/i4928;
    4992,  U4992/u4992,   I4992/i4992;
    5056,  U5056/u5056,   I5056/i5056;
    5120,  U5120/u5120,   I5120/i5120;
    5184,  U5184/u5184,   I5184/i5184;
    5248,  U5248/u5248,   I5248/i5248;
    5312,  U5312/u5312,   I5312/i5312;
    5376,  U5376/u5376,   I5376/i5376;
    5440,  U5440/u5440,   I5440/i5440;
    5504,  U5504/u5504,   I5504/i5504;
    5568,  U5568/u5568,   I5568/i5568;
    5632,  U5632/u5632,   I5632/i5632;
    5696,  U5696/u5696,   I5696/i5696;
    5760,  U5760/u5760,   I5760/i5760;
    5824,  U5824/u5824,   I5824/i5824;
    5888,  U5888/u5888,   I5888/i5888;
    5952,  U5952/u5952,   I5952/i5952;
    6016,  U6016/u6016,   I6016/i6016;
    6080,  U6080/u6080,   I6080/i6080;
    6144,  U6144/u6144,   I6144/i6144;
    6208,  U6208/u6208,   I6208/i6208;
    6272,  U6272/u6272,   I6272/i6272;
    6336,  U6336/u6336,   I6336/i6336;
    6400,  U6400/u6400,   I6400/i6400;
    6464,  U6464/u6464,   I6464/i6464;
    6528,  U6528/u6528,   I6528/i6528;
    6592,  U6592/u6592,   I6592/i6592;
    6656,  U6656/u6656,   I6656/i6656;
    6720,  U6720/u6720,   I6720/i6720;
    6784,  U6784/u6784,   I6784/i6784;
    6848,  U6848/u6848,   I6848/i6848;
    6912,  U6912/u6912,   I6912/i6912;
    6976,  U6976/u6976,   I6976/i6976;
    7040,  U7040/u7040,   I7040/i7040;
    7104,  U7104/u7104,   I7104/i7104;
    7168,  U7168/u7168,   I7168/i7168;
    7232,  U7232/u7232,   I7232/i7232;
    7296,  U7296/u7296,   I7296/i7296;
    7360,  U7360/u7360,   I7360/i7360;
    7424,  U7424/u7424,   I7424/i7424;
    7488,  U7488/u7488,   I7488/i7488;
    7552,  U7552/u7552,   I7552/i7552;
    7616,  U7616/u7616,   I7616/i7616;
    7680,  U7680/u7680,   I7680/i7680;
    7744,  U7744/u7744,   I7744/i7744;
    7808,  U7808/u7808,   I7808/i7808;
    7872,  U7872/u7872,   I7872/i7872;
    7936,  U7936/u7936,   I7936/i7936;
    8000,  U8000/u8000,   I8000/i8000;
    8064,  U8064/u8064,   I8064/i8064;
    8128,  U8128/u8128,   I8128/i8128;
);

