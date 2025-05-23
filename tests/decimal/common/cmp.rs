macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use std::cmp::{max, min};
            use fastnum::{$dec, $D};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use std::cmp::{max, min};
            use fastnum::{$dec, $D};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000), $dec!(1.41421356237309504880168872420969807856967187537694807317667974000000000))]
        #[case($dec!(1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000), $dec!(11.41421356237309504880168872420969807856967187537694807317667974000000000))]
        fn test_cmp_512(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(a <= b);
            assert!(b > a);
            assert!(b >= a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(-1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000), $dec!(-11.41421356237309504880168872420969807856967187537694807317667974000000000e-2))]
        #[case($dec!(-11.41421356237309504880168872420969807856967187537694807317667974000000000), $dec!(-1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000))]
        fn test_cmp_signed_512(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(a <= b);
            assert!(b > a);
            assert!(b >= a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    };

    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(472697816888807260.1604), $dec!(472697816888807260.16040000000000000000001))]
        #[case($dec!(1), $dec!(1.0000000000000000000000000000000000000000000000000001))]
        #[case($dec!(1000000000000000000000000000000000000000), $dec!(1e41))]
        #[case($dec!(1116386634271380982470843247639640260491505327092723527088459), $dec!(759522625769651746138617259189939751893902453291243506584717e2))]
        fn test_cmp_256(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(a <= b);
            assert!(b > a);
            assert!(b >= a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(-1.0000000000000000000000000000000000000000000000000001), $dec!(-1))]
        fn test_cmp_signed_256(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(a <= b);
            assert!(b > a);
            assert!(b >= a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(1))]
        #[case($dec!(1), $dec!(2))]
        #[case($dec!(1), $dec!(10))]
        #[case($dec!(2), $dec!(3))]
        #[case($dec!(100), $dec!(100.1))]
        #[case($dec!(1e2), $dec!(100.1))]
        #[case($dec!(100), $dec!(1.1e2))]
        #[case($dec!(1.2345), $dec!(1.2346))]
        #[case($dec!(25.6), $dec!(25.8))]
        #[case($dec!(1e-9223), $dec!(1))]
        #[case($dec!(1e-9223), $dec!(1e9223))]
        #[case($dec!(1), $dec!(340282366920938463463374607431768211455))]
        #[case($dec!(500), $dec!(51e1))]
        #[case($dec!(44e1), $dec!(500))]
        #[case($dec!(1234000000000), $dec!(12345e9))]
        #[case($dec!(1514932018891593916341142774e-24), $dec!(1514932018891593916341142773.0001))]
        #[case($dec!(1e-32767), $dec!(1e32767))]
        #[case($dec!(2), $dec!(0.2e2))]
        #[case($dec!(1e-900), $dec!(1e45))]
        #[case($dec!(1e-900), $dec!(1e+900))]
        #[case($dec!(35_000), $dec!(192051.716384861040546579908080116424889))]
        #[case($dec!(35e-80), $dec!(19205.1716384861040546579908080116424889))]
        #[case($dec!(19205.1716384861040546579908080116424889), $dec!(35_000))]
        #[case($dec!(192051.716384861040546579908080116424889), $dec!(35e80))]
        #[case($dec!(35e-80), $dec!(192051.716384861040546579908080116424889))]
        fn test_cmp(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(a <= b);
            assert!(b > a);
            assert!(b >= a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }

        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0))]
        #[case($dec!(00), $dec!(0))]
        #[case($dec!(0), $dec!(0.00))]
        #[case($dec!(1), $dec!(1))]
        #[case($dec!(00001), $dec!(1))]
        #[case($dec!(00001), $dec!(1.0000))]
        #[case($dec!(1), $dec!(1.00))]
        #[case($dec!(10), $dec!(10))]
        #[case($dec!(10), $dec!(1e1))]
        #[case($dec!(1.1), $dec!(1.1))]
        #[case($dec!(1.2e-2), $dec!(0.012))]
        #[case($dec!(5000), $dec!(50e2))]
        #[case($dec!(0.000034500), $dec!(345e-7))]
        #[case($dec!(1514932018891593.916341142773), $dec!(1514932018891593916341142773e-12))]
        #[case($dec!(1e32768), $dec!(1e32768))]
        #[case($dec!(1e-32767), $dec!(1e-32767))]
        #[case($dec!(2), $dec!(0.2e1))]
        #[case($dec!(0e1), $dec!(0.0))]
        #[case($dec!(0e1), $dec!(0.0))]
        #[case($dec!(0e0), $dec!(0.0))]
        #[case($dec!(0e-0), $dec!(0.0))]
        #[case($dec!(0901300e-3), $dec!(901.3))]
        #[case($dec!(0.901300e+3), $dec!(901.3))]
        #[case($dec!(0e-1), $dec!(0.0))]
        #[case($dec!(2123121e1231), $dec!(212.3121e1235))]
        fn test_eq(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_eq!(a, b);

            assert!(!(a < b));
            assert!(!(b > a));

            assert!(a <= b);
            assert!(a >= b);

            assert_eq!(b, a);
            assert!(a <= b);
            assert!(a >= b);
        }

        #[rstest(::trace)]
        #[case($D::INFINITY, $D::INFINITY)]
        fn test_eq_special(#[case] a: $D, #[case] b: $D) {
             assert_eq!(a, b);
        }

        #[rstest(::trace)]
        #[case($D::NAN, $D::NAN)]
        #[case($D::ONE, $D::NAN)]
        #[case($D::NAN, $D::ONE)]
        #[case($D::INFINITY, $D::NAN)]
        fn test_ne_special(#[case] a: $D, #[case] b: $D) {
            assert_ne!(a, b);
        }

        #[rstest(::trace)]
        #[case($D::ZERO, $D::ONE)]
        #[case($D::ONE, $D::MAX)]
        #[case($D::MAX, $D::INFINITY)]
        #[case($D::INFINITY, $D::NAN)]
        fn test_cmp_special(#[case] a: $D, #[case] b: $D) {
            assert!(a < b);
        }

        #[rstest(::trace)]
        fn test_sort() {
            let mut positions = vec![
                $dec!(2.0),
                $dec!(4.0),
                $dec!(1.0),
                $dec!(5.0),
                $dec!(3.0),
                $dec!(3.0),
                $D::INFINITY,
                $D::ZERO,
            ];

            positions.sort_by(|a, b| a.cmp(&b));
            itertools::assert_equal(positions, vec![
                $D::ZERO,
                $dec!(1.0),
                $dec!(2.0),
                $dec!(3.0),
                $dec!(3.0),
                $dec!(4.0),
                $dec!(5.0),
                $D::INFINITY,
            ]);
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {

    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $dec!(0))]
        #[case($dec!(-0), $dec!(+0))]
        #[case($dec!(-1), $dec!(1))]
        #[case($dec!(-1), $dec!(0))]
        #[case($dec!(-1), $dec!(-0))]
        #[case($dec!(-1), $dec!(+0))]
        #[case($dec!(-10), $dec!(10))]
        #[case($dec!(-1), $dec!(10))]
        #[case($dec!(-1.1), $dec!(1.01))]
        #[case($dec!(-0.001), $dec!(-0.000000001))]
        #[case($dec!(-0.001), $dec!(+0.000000001))]
        fn test_cmp_signed(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a <= b);
            assert!(a < b);
            assert!(b > a);
            assert!(b >= a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }

        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0))]
        #[case($dec!(0), $dec!(+0))]
        #[case($dec!(-0), $dec!(-0))]
        #[case($dec!(+0), $dec!(+0))]
        #[case($dec!(+1.1), $dec!(+1.1))]
        #[case($dec!(-1.1), $dec!(-1.1))]
        #[case($dec!(-1), $dec!(-1000e-3))]
        #[case($dec!(-0.000034500), $dec!(-345e-7))]
        fn test_eq_signed(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_eq!(a, b);
            assert_eq!(b, a);
        }

        #[rstest(::trace)]
        #[case($D::NEG_INFINITY, $D::NEG_INFINITY)]
        fn test_eq_special_signed(#[case] a: $D, #[case] b: $D) {
             assert_eq!(a, b);
        }

        #[rstest(::trace)]
        #[case($D::NEG_INFINITY, $D::NAN)]
        fn test_ne_special_signed(#[case] a: $D, #[case] b: $D) {
            assert_ne!(a, b);
        }

        #[rstest(::trace)]
        #[case($D::NEG_INFINITY, $D::MIN)]
        #[case($D::MIN, $D::ONE.neg())]
        #[case($D::ONE.neg(), $D::ZERO.neg())]
        #[case($D::ZERO.neg(), $D::ZERO)]
        fn test_cmp_special_signed(#[case] a: $D, #[case] b: $D) {
            assert!(a < b);
        }

        #[rstest(::trace)]
        fn test_sort_signed() {
            let mut positions = vec![
                $dec!(2.0),
                $dec!(4.0),
                $dec!(1.0),
                $dec!(5.0),
                $dec!(3.0),
                $dec!(3.0),
                $D::NEG_INFINITY,
                $D::INFINITY,
                $D::ZERO,
                $dec!(-1.0),
                $dec!(-5.0),
                $dec!(-4.0),
                $dec!(-2.0),
                $dec!(-3.0),
                $D::ZERO.neg(),
            ];

            positions.sort_by(|a, b| a.cmp(&b));
            itertools::assert_equal(positions, vec![
                $D::NEG_INFINITY,
                $dec!(-5.0),
                $dec!(-4.0),
                $dec!(-3.0),
                $dec!(-2.0),
                $dec!(-1.0),
                $D::ZERO.neg(),
                $D::ZERO,
                $dec!(1.0),
                $dec!(2.0),
                $dec!(3.0),
                $dec!(3.0),
                $dec!(4.0),
                $dec!(5.0),
                $D::INFINITY,
            ]);
        }
    };
}

pub(crate) use test_impl;
