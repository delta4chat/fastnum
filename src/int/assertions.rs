use crate::{utils::assert_eq_size, *};

macro_rules! assert_impls {
    ($($uint:ident, $bits:literal;)*) => {
        const _: () = {
            $(
                assert!(($bits % 64) == 0);
                assert_eq_size!($uint, [u64; $bits / 64]);
            )*
        };
    }
}

assert_impls!(
    //U64, 64;
    U128, 128;
    U256, 256;
    U512, 512;
    U1024, 1024;
    U2048, 2048;
    U4096, 4096;
    U8192, 8192;
);
