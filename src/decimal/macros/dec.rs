macro_rules! macro_impl {
    ($d:tt, $DEC: ident, $bits: literal, $sign: ident, $name: ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct ", $bits, "-bit ", stringify!($sign), " [`", stringify!($DEC), "`](crate::", stringify!($DEC), ") decimal from literals in compile time.")]
        ///
        /// Const-evaluated in compile time macro-helper can be used for definitions of constants or variables whose value is known in compile time.
        ///
        /// # Examples:
        ///
        /// Basic usage:
        ///
        /// ```
        /// use fastnum::*;
        ///
        #[doc = concat!("const N: ", stringify!($DEC), " = ", stringify!($name), "!(1.23456789);")]
        /// assert!(!N.is_zero());
        ///
        #[doc = concat!("let num = ", stringify!($name), "!(0);")]
        /// assert!(num.is_zero());
        ///
        #[doc = concat!("const A: ", stringify!($DEC), " = ", stringify!($name), "!(5);")]
        #[doc = concat!("const B: ", stringify!($DEC), " = ", stringify!($name), "!(1_000);")]
        #[doc = concat!("const C: ", stringify!($DEC), " = A.div(B);")]
        ///
        #[doc = concat!("assert_eq!(C, ", stringify!($name), "!(0.005));")]
        ///
        /// ```
        ///
        /// ## Static assertions:
        ///
        /// ```compile_fail
        /// // The below example will fail to compile, as the function will panic at compile time:
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($DEC), "};")]
        ///
        /// // Gives a compile error of "error[E0080]: evaluation of constant value failed...
        /// // the evaluated program panicked at 'attempt to parse decimal from string containing invalid digit'",
        #[doc = concat!("const N: ", stringify!($DEC), " = ", stringify!($name), "!(A1.23456789);")]
        /// ```
        ///
        /// This allows you to perform all the necessary checks such as potentialy overflow or calculation accuracy loss and others at the compile time.
        /// Protect from unexpected errors in runtime.
        ///
        macro_rules! $name {
            ($d($d body:tt)*) => {{
                const __CTX: $crate::decimal::Context = $crate::decimal::Context::default();
                const __DECIMAL: $crate::$DEC = $crate::$DEC::parse_str(concat!($d(stringify!($d body)),*), __CTX);
                __DECIMAL
            }};
        }
    };
}

macro_impl!($, UD64,   64,   unsigned,   udec64);
macro_impl!($,  D64,   64,     signed,    dec64);

macro_impl!($, UD128,  128,  unsigned,  udec128);
macro_impl!($,  D128,  128,    signed,   dec128);

macro_impl!($, UD256,  256,  unsigned,  udec256);
macro_impl!($,  D256,  256,    signed,   dec256);

macro_impl!($, UD512,  512,  unsigned,  udec512);
macro_impl!($,  D512,  512,    signed,   dec512);

macro_impl!($, UD1024, 1024, unsigned, udec1024);
macro_impl!($,  D1024, 1024,   signed,  dec1024);

macro_impl!($, UD2048, 2048, unsigned, udec2048);
macro_impl!($,  D2048, 2048,   signed,  dec2048);

macro_impl!($, UD4096, 4096, unsigned, udec4096);
macro_impl!($,  D4096, 4096,   signed,  dec4096);

macro_impl!($, UD8192, 8192, unsigned, udec8192);
macro_impl!($,  D8192, 8192,   signed,  dec8192);

#[cfg(feature="more-types")]
mod extras {
    macro_impl!($,  UD192,  192, unsigned, udec192);
    macro_impl!($,   D192,  192,   signed,  dec192);

    macro_impl!($,  UD320,  320, unsigned, udec320);
    macro_impl!($,   D320,  320,   signed,  dec320);

    macro_impl!($,  UD384,  384, unsigned, udec384);
    macro_impl!($,   D384,  384,   signed,  dec384);

    macro_impl!($,  UD448,  448, unsigned, udec448);
    macro_impl!($,   D448,  448,   signed,  dec448);

    macro_impl!($,  UD576,  576, unsigned, udec576);
    macro_impl!($,   D576,  576,   signed,  dec576);

    macro_impl!($,  UD640,  640, unsigned, udec640);
    macro_impl!($,   D640,  640,   signed,  dec640);

    macro_impl!($,  UD704,  704, unsigned, udec704);
    macro_impl!($,   D704,  704,   signed,  dec704);

    macro_impl!($,  UD768,  768, unsigned, udec768);
    macro_impl!($,   D768,  768,   signed,  dec768);

    macro_impl!($,  UD832,  832, unsigned, udec832);
    macro_impl!($,   D832,  832,   signed,  dec832);

    macro_impl!($,  UD896,  896, unsigned, udec896);
    macro_impl!($,   D896,  896,   signed,  dec896);

    macro_impl!($,  UD960,  960, unsigned, udec960);
    macro_impl!($,   D960,  960,   signed,  dec960);

    macro_impl!($, UD1088, 1088, unsigned, udec1088);
    macro_impl!($,  D1088, 1088,   signed,  dec1088);

    macro_impl!($, UD1152, 1152, unsigned, udec1152);
    macro_impl!($,  D1152, 1152,   signed,  dec1152);

    macro_impl!($, UD1216, 1216, unsigned, udec1216);
    macro_impl!($,  D1216, 1216,   signed,  dec1216);

    macro_impl!($, UD1280, 1280, unsigned, udec1280);
    macro_impl!($,  D1280, 1280,   signed,  dec1280);

    macro_impl!($, UD1344, 1344, unsigned, udec1344);
    macro_impl!($,  D1344, 1344,   signed,  dec1344);

    macro_impl!($, UD1408, 1408, unsigned, udec1408);
    macro_impl!($,  D1408, 1408,   signed,  dec1408);

    macro_impl!($, UD1472, 1472, unsigned, udec1472);
    macro_impl!($,  D1472, 1472,   signed,  dec1472);

    macro_impl!($, UD1536, 1536, unsigned, udec1536);
    macro_impl!($,  D1536, 1536,   signed,  dec1536);

    macro_impl!($, UD1600, 1600, unsigned, udec1600);
    macro_impl!($,  D1600, 1600,   signed,  dec1600);

    macro_impl!($, UD1664, 1664, unsigned, udec1664);
    macro_impl!($,  D1664, 1664,   signed,  dec1664);

    macro_impl!($, UD1728, 1728, unsigned, udec1728);
    macro_impl!($,  D1728, 1728,   signed,  dec1728);

    macro_impl!($, UD1792, 1792, unsigned, udec1792);
    macro_impl!($,  D1792, 1792,   signed,  dec1792);

    macro_impl!($, UD1856, 1856, unsigned, udec1856);
    macro_impl!($,  D1856, 1856,   signed,  dec1856);

    macro_impl!($, UD1920, 1920, unsigned, udec1920);
    macro_impl!($,  D1920, 1920,   signed,  dec1920);

    macro_impl!($, UD1984, 1984, unsigned, udec1984);
    macro_impl!($,  D1984, 1984,   signed,  dec1984);

    macro_impl!($, UD2112, 2112, unsigned, udec2112);
    macro_impl!($,  D2112, 2112,   signed,  dec2112);

    macro_impl!($, UD2176, 2176, unsigned, udec2176);
    macro_impl!($,  D2176, 2176,   signed,  dec2176);

    macro_impl!($, UD2240, 2240, unsigned, udec2240);
    macro_impl!($,  D2240, 2240,   signed,  dec2240);

    macro_impl!($, UD2304, 2304, unsigned, udec2304);
    macro_impl!($,  D2304, 2304,   signed,  dec2304);

    macro_impl!($, UD2368, 2368, unsigned, udec2368);
    macro_impl!($,  D2368, 2368,   signed,  dec2368);

    macro_impl!($, UD2432, 2432, unsigned, udec2432);
    macro_impl!($,  D2432, 2432,   signed,  dec2432);

    macro_impl!($, UD2496, 2496, unsigned, udec2496);
    macro_impl!($,  D2496, 2496,   signed,  dec2496);

    macro_impl!($, UD2560, 2560, unsigned, udec2560);
    macro_impl!($,  D2560, 2560,   signed,  dec2560);

    macro_impl!($, UD2624, 2624, unsigned, udec2624);
    macro_impl!($,  D2624, 2624,   signed,  dec2624);

    macro_impl!($, UD2688, 2688, unsigned, udec2688);
    macro_impl!($,  D2688, 2688,   signed,  dec2688);

    macro_impl!($, UD2752, 2752, unsigned, udec2752);
    macro_impl!($,  D2752, 2752,   signed,  dec2752);

    macro_impl!($, UD2816, 2816, unsigned, udec2816);
    macro_impl!($,  D2816, 2816,   signed,  dec2816);

    macro_impl!($, UD2880, 2880, unsigned, udec2880);
    macro_impl!($,  D2880, 2880,   signed,  dec2880);

    macro_impl!($, UD2944, 2944, unsigned, udec2944);
    macro_impl!($,  D2944, 2944,   signed,  dec2944);

    macro_impl!($, UD3008, 3008, unsigned, udec3008);
    macro_impl!($,  D3008, 3008,   signed,  dec3008);

    macro_impl!($, UD3072, 3072, unsigned, udec3072);
    macro_impl!($,  D3072, 3072,   signed,  dec3072);

    macro_impl!($, UD3136, 3136, unsigned, udec3136);
    macro_impl!($,  D3136, 3136,   signed,  dec3136);

    macro_impl!($, UD3200, 3200, unsigned, udec3200);
    macro_impl!($,  D3200, 3200,   signed,  dec3200);

    macro_impl!($, UD3264, 3264, unsigned, udec3264);
    macro_impl!($,  D3264, 3264,   signed,  dec3264);

    macro_impl!($, UD3328, 3328, unsigned, udec3328);
    macro_impl!($,  D3328, 3328,   signed,  dec3328);

    macro_impl!($, UD3392, 3392, unsigned, udec3392);
    macro_impl!($,  D3392, 3392,   signed,  dec3392);

    macro_impl!($, UD3456, 3456, unsigned, udec3456);
    macro_impl!($,  D3456, 3456,   signed,  dec3456);

    macro_impl!($, UD3520, 3520, unsigned, udec3520);
    macro_impl!($,  D3520, 3520,   signed,  dec3520);

    macro_impl!($, UD3584, 3584, unsigned, udec3584);
    macro_impl!($,  D3584, 3584,   signed,  dec3584);

    macro_impl!($, UD3648, 3648, unsigned, udec3648);
    macro_impl!($,  D3648, 3648,   signed,  dec3648);

    macro_impl!($, UD3712, 3712, unsigned, udec3712);
    macro_impl!($,  D3712, 3712,   signed,  dec3712);

    macro_impl!($, UD3776, 3776, unsigned, udec3776);
    macro_impl!($,  D3776, 3776,   signed,  dec3776);

    macro_impl!($, UD3840, 3840, unsigned, udec3840);
    macro_impl!($,  D3840, 3840,   signed,  dec3840);

    macro_impl!($, UD3904, 3904, unsigned, udec3904);
    macro_impl!($,  D3904, 3904,   signed,  dec3904);

    macro_impl!($, UD3968, 3968, unsigned, udec3968);
    macro_impl!($,  D3968, 3968,   signed,  dec3968);

    macro_impl!($, UD4032, 4032, unsigned, udec4032);
    macro_impl!($,  D4032, 4032,   signed,  dec4032);

    macro_impl!($, UD4160, 4160, unsigned, udec4160);
    macro_impl!($,  D4160, 4160,   signed,  dec4160);

    macro_impl!($, UD4224, 4224, unsigned, udec4224);
    macro_impl!($,  D4224, 4224,   signed,  dec4224);

    macro_impl!($, UD4288, 4288, unsigned, udec4288);
    macro_impl!($,  D4288, 4288,   signed,  dec4288);

    macro_impl!($, UD4352, 4352, unsigned, udec4352);
    macro_impl!($,  D4352, 4352,   signed,  dec4352);

    macro_impl!($, UD4416, 4416, unsigned, udec4416);
    macro_impl!($,  D4416, 4416,   signed,  dec4416);

    macro_impl!($, UD4480, 4480, unsigned, udec4480);
    macro_impl!($,  D4480, 4480,   signed,  dec4480);

    macro_impl!($, UD4544, 4544, unsigned, udec4544);
    macro_impl!($,  D4544, 4544,   signed,  dec4544);

    macro_impl!($, UD4608, 4608, unsigned, udec4608);
    macro_impl!($,  D4608, 4608,   signed,  dec4608);

    macro_impl!($, UD4672, 4672, unsigned, udec4672);
    macro_impl!($,  D4672, 4672,   signed,  dec4672);

    macro_impl!($, UD4736, 4736, unsigned, udec4736);
    macro_impl!($,  D4736, 4736,   signed,  dec4736);

    macro_impl!($, UD4800, 4800, unsigned, udec4800);
    macro_impl!($,  D4800, 4800,   signed,  dec4800);

    macro_impl!($, UD4864, 4864, unsigned, udec4864);
    macro_impl!($,  D4864, 4864,   signed,  dec4864);

    macro_impl!($, UD4928, 4928, unsigned, udec4928);
    macro_impl!($,  D4928, 4928,   signed,  dec4928);

    macro_impl!($, UD4992, 4992, unsigned, udec4992);
    macro_impl!($,  D4992, 4992,   signed,  dec4992);

    macro_impl!($, UD5056, 5056, unsigned, udec5056);
    macro_impl!($,  D5056, 5056,   signed,  dec5056);

    macro_impl!($, UD5120, 5120, unsigned, udec5120);
    macro_impl!($,  D5120, 5120,   signed,  dec5120);

    macro_impl!($, UD5184, 5184, unsigned, udec5184);
    macro_impl!($,  D5184, 5184,   signed,  dec5184);

    macro_impl!($, UD5248, 5248, unsigned, udec5248);
    macro_impl!($,  D5248, 5248,   signed,  dec5248);

    macro_impl!($, UD5312, 5312, unsigned, udec5312);
    macro_impl!($,  D5312, 5312,   signed,  dec5312);

    macro_impl!($, UD5376, 5376, unsigned, udec5376);
    macro_impl!($,  D5376, 5376,   signed,  dec5376);

    macro_impl!($, UD5440, 5440, unsigned, udec5440);
    macro_impl!($,  D5440, 5440,   signed,  dec5440);

    macro_impl!($, UD5504, 5504, unsigned, udec5504);
    macro_impl!($,  D5504, 5504,   signed,  dec5504);

    macro_impl!($, UD5568, 5568, unsigned, udec5568);
    macro_impl!($,  D5568, 5568,   signed,  dec5568);

    macro_impl!($, UD5632, 5632, unsigned, udec5632);
    macro_impl!($,  D5632, 5632,   signed,  dec5632);

    macro_impl!($, UD5696, 5696, unsigned, udec5696);
    macro_impl!($,  D5696, 5696,   signed,  dec5696);

    macro_impl!($, UD5760, 5760, unsigned, udec5760);
    macro_impl!($,  D5760, 5760,   signed,  dec5760);

    macro_impl!($, UD5824, 5824, unsigned, udec5824);
    macro_impl!($,  D5824, 5824,   signed,  dec5824);

    macro_impl!($, UD5888, 5888, unsigned, udec5888);
    macro_impl!($,  D5888, 5888,   signed,  dec5888);

    macro_impl!($, UD5952, 5952, unsigned, udec5952);
    macro_impl!($,  D5952, 5952,   signed,  dec5952);

    macro_impl!($, UD6016, 6016, unsigned, udec6016);
    macro_impl!($,  D6016, 6016,   signed,  dec6016);

    macro_impl!($, UD6080, 6080, unsigned, udec6080);
    macro_impl!($,  D6080, 6080,   signed,  dec6080);

    macro_impl!($, UD6144, 6144, unsigned, udec6144);
    macro_impl!($,  D6144, 6144,   signed,  dec6144);

    macro_impl!($, UD6208, 6208, unsigned, udec6208);
    macro_impl!($,  D6208, 6208,   signed,  dec6208);

    macro_impl!($, UD6272, 6272, unsigned, udec6272);
    macro_impl!($,  D6272, 6272,   signed,  dec6272);

    macro_impl!($, UD6336, 6336, unsigned, udec6336);
    macro_impl!($,  D6336, 6336,   signed,  dec6336);

    macro_impl!($, UD6400, 6400, unsigned, udec6400);
    macro_impl!($,  D6400, 6400,   signed,  dec6400);

    macro_impl!($, UD6464, 6464, unsigned, udec6464);
    macro_impl!($,  D6464, 6464,   signed,  dec6464);

    macro_impl!($, UD6528, 6528, unsigned, udec6528);
    macro_impl!($,  D6528, 6528,   signed,  dec6528);

    macro_impl!($, UD6592, 6592, unsigned, udec6592);
    macro_impl!($,  D6592, 6592,   signed,  dec6592);

    macro_impl!($, UD6656, 6656, unsigned, udec6656);
    macro_impl!($,  D6656, 6656,   signed,  dec6656);

    macro_impl!($, UD6720, 6720, unsigned, udec6720);
    macro_impl!($,  D6720, 6720,   signed,  dec6720);

    macro_impl!($, UD6784, 6784, unsigned, udec6784);
    macro_impl!($,  D6784, 6784,   signed,  dec6784);

    macro_impl!($, UD6848, 6848, unsigned, udec6848);
    macro_impl!($,  D6848, 6848,   signed,  dec6848);

    macro_impl!($, UD6912, 6912, unsigned, udec6912);
    macro_impl!($,  D6912, 6912,   signed,  dec6912);

    macro_impl!($, UD6976, 6976, unsigned, udec6976);
    macro_impl!($,  D6976, 6976,   signed,  dec6976);

    macro_impl!($, UD7040, 7040, unsigned, udec7040);
    macro_impl!($,  D7040, 7040,   signed,  dec7040);

    macro_impl!($, UD7104, 7104, unsigned, udec7104);
    macro_impl!($,  D7104, 7104,   signed,  dec7104);

    macro_impl!($, UD7168, 7168, unsigned, udec7168);
    macro_impl!($,  D7168, 7168,   signed,  dec7168);

    macro_impl!($, UD7232, 7232, unsigned, udec7232);
    macro_impl!($,  D7232, 7232,   signed,  dec7232);

    macro_impl!($, UD7296, 7296, unsigned, udec7296);
    macro_impl!($,  D7296, 7296,   signed,  dec7296);

    macro_impl!($, UD7360, 7360, unsigned, udec7360);
    macro_impl!($,  D7360, 7360,   signed,  dec7360);

    macro_impl!($, UD7424, 7424, unsigned, udec7424);
    macro_impl!($,  D7424, 7424,   signed,  dec7424);

    macro_impl!($, UD7488, 7488, unsigned, udec7488);
    macro_impl!($,  D7488, 7488,   signed,  dec7488);

    macro_impl!($, UD7552, 7552, unsigned, udec7552);
    macro_impl!($,  D7552, 7552,   signed,  dec7552);

    macro_impl!($, UD7616, 7616, unsigned, udec7616);
    macro_impl!($,  D7616, 7616,   signed,  dec7616);

    macro_impl!($, UD7680, 7680, unsigned, udec7680);
    macro_impl!($,  D7680, 7680,   signed,  dec7680);

    macro_impl!($, UD7744, 7744, unsigned, udec7744);
    macro_impl!($,  D7744, 7744,   signed,  dec7744);

    macro_impl!($, UD7808, 7808, unsigned, udec7808);
    macro_impl!($,  D7808, 7808,   signed,  dec7808);

    macro_impl!($, UD7872, 7872, unsigned, udec7872);
    macro_impl!($,  D7872, 7872,   signed,  dec7872);

    macro_impl!($, UD7936, 7936, unsigned, udec7936);
    macro_impl!($,  D7936, 7936,   signed,  dec7936);

    macro_impl!($, UD8000, 8000, unsigned, udec8000);
    macro_impl!($,  D8000, 8000,   signed,  dec8000);

    macro_impl!($, UD8064, 8064, unsigned, udec8064);
    macro_impl!($,  D8064, 8064,   signed,  dec8064);

    macro_impl!($, UD8128, 8128, unsigned, udec8128);
    macro_impl!($,  D8128, 8128,   signed,  dec8128);
}

