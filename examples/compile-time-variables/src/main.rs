use fastnum::{decimal::*, *};

/// Output:
///
/// ```text
/// x: 0.0135002097724903110032940511844876358848
/// x (rescaled): 0.013
/// x (rounded): 0.013
/// x (rescaled half-up): 0.014
/// x (rounded half-up): 0.014
/// ```
fn main() {
    assert_eq!((dec128!(1000) / dec128!(94000)).round(2), dec128!(0.01));

    let x = dec128!(0.0135002097724903110032940511844876358848);

    println!("x: {}", x);

    println!("x (rescaled): {}", x.rescale(3));
    println!("x (rounded): {}", x.round(3));

    println!(
        "x (rescaled half-up): {}",
        x.with_rounding_mode(RoundingMode::HalfUp).rescale(3)
    );
    println!(
        "x (rounded half-up): {}",
        x.with_rounding_mode(RoundingMode::HalfUp).round(3)
    );
}
