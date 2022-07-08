use rug::Float;

pub const F_PREC: u32 = 300;

lazy_static! {
    pub static ref EPSILON: Float = Float::with_val(F_PREC, Float::parse("1e-10").unwrap());
};

lazy_static! {
    pub static ref RELATIVE_TOLERANCE: Float =
        Float::with_val(F_PREC, Float::parse("1e-3").unwrap());
};

pub fn is_within_tolerance_absolute(a: Float, b: Float) -> bool {
    (a - b).abs() < *EPSILON
}

pub fn is_within_tolerance_relative(actual: Float, got: Float, rel_tol: f64) -> bool {
    // The smallest number representable is 1e-18
    if got.clone().abs() < 1e-15 {
        if actual.clone().abs() < 1e-15 {
            return true;
        }
    }
    (actual.clone() - got.clone()).abs() / got < rel_tol
}
