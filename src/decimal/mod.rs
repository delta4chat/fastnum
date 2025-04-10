//! # Decimal numbers

#[cfg(feature = "test-util")]
#[doc(hidden)]
pub mod extras;

#[cfg(not(feature = "test-util"))]
pub(crate) mod extras;

pub(crate) mod dec;
pub(crate) mod doc;
pub(crate) mod round;
pub(crate) mod udec;

mod context;
mod sign;
mod signals;

#[cfg(debug_assertions)]
mod assertions;

#[allow(dead_code)]
pub(crate) mod utils;

pub(crate) mod errors;

#[macro_use]
mod macros;

pub use context::{Context, Notation, FormatStyle, RoundingMode, SignalsTraps};
pub use dec::Decimal;
pub use errors::{DecimalError, ParseError};
pub use sign::Sign;
pub use signals::Signals;
pub use udec::UnsignedDecimal;

use crate::decimal::doc::decimal_type_doc;

macro_rules! decimal_types {
    ( $($bits: literal $u: ident $s: ident; ) *)  => {
        $(
            #[doc = decimal_type_doc!($bits, "unsigned")]
            pub type $u = UnsignedDecimal::<{$bits / 64}>;

            #[doc = decimal_type_doc!($bits, "signed")]
            pub type $s = Decimal::<{$bits / 64}>;
        )*
    };
}

decimal_types!(
    64   UD64   D64;
    128  UD128  D128;
    256  UD256  D256;
    512  UD512  D512;
    1024 UD1024 D1024;
    2048 UD2048 D2048;
    4096 UD4096 D4096;
    8192 UD8192 D8192;
);

#[cfg(feature="more-types")]
decimal_types!(
    192  UD192  D192;
    320  UD320  D320;
    384  UD384  D384;
    448  UD448  D448;
    576  UD576  D576;
    640  UD640  D640;
    704  UD704  D704;
    768  UD768  D768;
    832  UD832  D832;
    896  UD896  D896;
    960  UD960  D960;
    1088 UD1088 D1088;
    1152 UD1152 D1152;
    1216 UD1216 D1216;
    1280 UD1280 D1280;
    1344 UD1344 D1344;
    1408 UD1408 D1408;
    1472 UD1472 D1472;
    1536 UD1536 D1536;
    1600 UD1600 D1600;
    1664 UD1664 D1664;
    1728 UD1728 D1728;
    1792 UD1792 D1792;
    1856 UD1856 D1856;
    1920 UD1920 D1920;
    1984 UD1984 D1984;
    2112 UD2112 D2112;
    2176 UD2176 D2176;
    2240 UD2240 D2240;
    2304 UD2304 D2304;
    2368 UD2368 D2368;
    2432 UD2432 D2432;
    2496 UD2496 D2496;
    2560 UD2560 D2560;
    2624 UD2624 D2624;
    2688 UD2688 D2688;
    2752 UD2752 D2752;
    2816 UD2816 D2816;
    2880 UD2880 D2880;
    2944 UD2944 D2944;
    3008 UD3008 D3008;
    3072 UD3072 D3072;
    3136 UD3136 D3136;
    3200 UD3200 D3200;
    3264 UD3264 D3264;
    3328 UD3328 D3328;
    3392 UD3392 D3392;
    3456 UD3456 D3456;
    3520 UD3520 D3520;
    3584 UD3584 D3584;
    3648 UD3648 D3648;
    3712 UD3712 D3712;
    3776 UD3776 D3776;
    3840 UD3840 D3840;
    3904 UD3904 D3904;
    3968 UD3968 D3968;
    4032 UD4032 D4032;
    4160 UD4160 D4160;
    4224 UD4224 D4224;
    4288 UD4288 D4288;
    4352 UD4352 D4352;
    4416 UD4416 D4416;
    4480 UD4480 D4480;
    4544 UD4544 D4544;
    4608 UD4608 D4608;
    4672 UD4672 D4672;
    4736 UD4736 D4736;
    4800 UD4800 D4800;
    4864 UD4864 D4864;
    4928 UD4928 D4928;
    4992 UD4992 D4992;
    5056 UD5056 D5056;
    5120 UD5120 D5120;
    5184 UD5184 D5184;
    5248 UD5248 D5248;
    5312 UD5312 D5312;
    5376 UD5376 D5376;
    5440 UD5440 D5440;
    5504 UD5504 D5504;
    5568 UD5568 D5568;
    5632 UD5632 D5632;
    5696 UD5696 D5696;
    5760 UD5760 D5760;
    5824 UD5824 D5824;
    5888 UD5888 D5888;
    5952 UD5952 D5952;
    6016 UD6016 D6016;
    6080 UD6080 D6080;
    6144 UD6144 D6144;
    6208 UD6208 D6208;
    6272 UD6272 D6272;
    6336 UD6336 D6336;
    6400 UD6400 D6400;
    6464 UD6464 D6464;
    6528 UD6528 D6528;
    6592 UD6592 D6592;
    6656 UD6656 D6656;
    6720 UD6720 D6720;
    6784 UD6784 D6784;
    6848 UD6848 D6848;
    6912 UD6912 D6912;
    6976 UD6976 D6976;
    7040 UD7040 D7040;
    7104 UD7104 D7104;
    7168 UD7168 D7168;
    7232 UD7232 D7232;
    7296 UD7296 D7296;
    7360 UD7360 D7360;
    7424 UD7424 D7424;
    7488 UD7488 D7488;
    7552 UD7552 D7552;
    7616 UD7616 D7616;
    7680 UD7680 D7680;
    7744 UD7744 D7744;
    7808 UD7808 D7808;
    7872 UD7872 D7872;
    7936 UD7936 D7936;
    8000 UD8000 D8000;
    8064 UD8064 D8064;
    8128 UD8128 D8128;
);
