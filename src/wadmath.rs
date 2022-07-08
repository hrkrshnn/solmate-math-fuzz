use crate::float::F_PREC;
use rug::Float;

pub const WAD_SCALE: u128 = 10u128.pow(18);

pub fn to_wad<T: std::ops::Mul<T, Output = T> + std::convert::From<u128>>(a: T) -> T {
    a * T::from(WAD_SCALE)
}

pub fn wad_to_float<T: std::fmt::Display>(a: T) -> Float {
    Float::with_val(F_PREC, Float::parse(a.to_string()).unwrap()) / WAD_SCALE.clone()
}
