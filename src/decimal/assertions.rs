use crate::{utils::assert_eq_size, *};

macro_rules! assert_impls {
    ($($dec:ident, $udec:ident, $bits:literal;)*) => {
        const _: () = {
            $(
                assert!(($bits % 64) == 0);
                // + 1 (control block)
                // + 2 (context struct)
                assert_eq_size!($dec, [u64; { $bits / 64 } + 1 + 2]);
                assert_eq_size!($udec, $dec);
            )*
        };
    };
}

assert_impls!(
    D128, UD128, 128;
    D256, UD256, 256;
    D512, UD512, 512;
    D1024, UD1024, 1024;
);
