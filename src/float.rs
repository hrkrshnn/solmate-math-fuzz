use rug::Float;

pub const F_PREC: u32 = 300;

lazy_static!(
    pub static ref EPSILON: Float = Float::with_val(
        F_PREC,
        Float::parse("1e-10").unwrap()
    );
);

