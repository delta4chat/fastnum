use fastnum::*;

/// Example output
///
/// ```text
/// -Inf
/// -0
/// 0
/// 1.0
/// 2.0
/// 3.0
/// 3.0
/// 4.0
/// 5.0
/// Inf
/// NaN
/// ```
fn main() {
    let mut positions = vec![
        dec128!(2.0),
        dec128!(4.0),
        dec128!(1.0),
        dec128!(5.0),
        dec128!(3.0),
        dec128!(3.0),
        D128::NAN,
        D128::NEG_INFINITY,
        D128::INFINITY,
        D128::ZERO,
        D128::ZERO.neg(),
    ];

    positions.sort_by(|a, b| a.cmp(&b));

    for i in positions {
        println!("{i}");
    }
}
