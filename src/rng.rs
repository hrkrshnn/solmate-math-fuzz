use crate::wadmath::WAD_SCALE;

use ethers::core::types::{I256, U256};

use rand::distributions::{Distribution, Standard};
use rand::prelude::ThreadRng;
use rand::Rng;

lazy_static! {
    static ref WADMAX: U256 = U256::from("0x12725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f21");
};

// TODO can we avoid this wrapping?
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct WU256(pub U256);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct WI256(pub I256);

impl Distribution<WU256> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WU256 {
        let arr: [u64; 4] = rng.gen();
        WU256(U256(arr))
    }
}

impl Distribution<WI256> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WI256 {
        let arr: [u64; 4] = rng.gen();
        WI256(I256::from_raw(U256(arr)))
    }
}

// Rejection sampling is bad due to the range being rather large :(
// This is purely a hack
// TODO what can we say about the distribution?
pub fn gen_wad(mut rng: ThreadRng) -> U256 {
    let num: WU256 = rng.gen();
    num.0 / WAD_SCALE
}

pub fn gen_nonzero_signed_wad(rng: &mut ThreadRng) -> I256 {
    let mut num: WI256 = rng.gen();
    while num.0.is_negative() {
        num = rng.gen();
    }
    num.0 / I256::from(WAD_SCALE)
}

pub fn gen_wad_for_exp(rng: &mut ThreadRng) -> I256 {
    let beg: i128 = -135305999368893231589;
    let end: i128 = 135305999368893231589;

    let num: i128 = rng.gen_range(beg..=end);
    I256::from(num)
}

#[test]
fn test_sample() {
    use rand::thread_rng;
    let mut rng = thread_rng();
    let num: WU256 = rng.gen();
    println!("Random sample: {:?}", num);
}

#[test]
fn test_wad_max() {
    println!("WADMAX: {}", *WADMAX);
}
