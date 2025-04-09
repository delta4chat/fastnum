use std::{
    io::Write,
    env,
    path::PathBuf,
};

const DEFAULT_ROUNDING_MODE: &'static str = "HalfUp";
const FMT_EXPONENTIAL_LOWER_THRESHOLD: &'static str = "5";
const FMT_EXPONENTIAL_UPPER_THRESHOLD: &'static str = "15";
const FMT_MAX_INTEGER_PADDING: &'static str = "1000";
const SERDE_DESERIALIZE_MODE: &'static str = "Strict";

fn main() {
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let file = std::fs::File::create(out_dir.join("config.rs")).unwrap();
    write_config(file);
}

macro_rules! load_env {
    ($env:ident, $name:literal, $default:ident) => {{
        println!("cargo:rerun-if-env-changed={}", $name);
        $env::var($name).unwrap_or_else(|_| $default.to_owned())
    }};
}

fn write_config<W: Write>(mut out: W) {
    let rounding_mode_name = load_env!(
        env,
        "RUST_FASTNUM_DEFAULT_ROUNDING_MODE",
        DEFAULT_ROUNDING_MODE
    );

    let low_value = load_env!(
        env,
        "RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD",
        FMT_EXPONENTIAL_LOWER_THRESHOLD
    );
    let high_value = load_env!(
        env,
        "RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD",
        FMT_EXPONENTIAL_UPPER_THRESHOLD
    );
    let max_padding = load_env!(
        env,
        "RUST_FASTNUM_FMT_MAX_INTEGER_PADDING",
        FMT_MAX_INTEGER_PADDING
    );
    let mode = load_env!(
        env,
        "RUST_FASTNUM_SERDE_DESERIALIZE_MODE",
        SERDE_DESERIALIZE_MODE
    );

    let low_value: u32 = low_value
        .parse::<std::num::NonZeroU32>()
        .expect("$RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD must be an integer > 0")
        .into();

    let high_value: u32 = high_value
        .parse::<u32>()
        .expect("$RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD must be valid u32");

    let max_padding: u32 = max_padding
        .parse::<u32>()
        .expect("$RUST_FASTNUM_FMT_MAX_INTEGER_PADDING must be valid u32");

    write!(
        out,
r#"
use crate::decimal::RoundingMode;

#[cfg(feature="serde")]
use crate::decimal::extras::serde::DeserializeMode;
#[cfg(feature="serde")]
pub(crate) const SERDE_DESERIALIZE_MODE: DeserializeMode = DeserializeMode::{mode};

pub(crate) const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::{rounding_mode_name};

pub(crate) const EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD: u32 = {low_value};
pub(crate) const EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD: u32 = {high_value};

pub(crate) const FMT_MAX_INTEGER_PADDING: u32 = {max_padding};
"#
    ).unwrap();
}
