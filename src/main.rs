mod rng;
mod wadmath;
mod float;

use rng::{gen_nonzero_signed_wad};
use wadmath::{to_wad, wad_to_float};
use float::{F_PREC, EPSILON};

use ethers::prelude::*;

use ethers::{utils::Anvil};

use eyre::Result;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use tokio;
use rand::{thread_rng};
use rug::Float;

#[macro_use]
extern crate lazy_static;

abigen!(Fuzz, "./out/Fuzz.sol/Fuzz.json");

type FuzzType = Fuzz<SignerMiddleware<Provider<Http>, LocalWallet>>;

const FUZZ_RUNS: u64 = 100;

async fn test_name(fuzz: &FuzzType) -> Result<()> {
    let name = fuzz.name().call().await?;
    assert_eq!(name, "Fuzz");
    Ok(())
}

async fn test_add(fuzz: &FuzzType) -> Result<()> {
    let sum = fuzz.add(1.into(), 2.into()).call().await?;
    assert_eq!(sum, 3.into());
    Ok(())
}

async fn test_ln(fuzz: &FuzzType) -> Result<()> {
    // Testing for ln(1)
    let ln = fuzz.ln(to_wad(1.into())).call().await?;
    assert_eq!(ln, 0.into());

    let f = Float::with_val(F_PREC, 999);
    let rln = f.ln();
    println!("rust-ln: {}", rln);
    let sln = fuzz.ln(to_wad(999.into())).call().await?;
    let sln = wad_to_float(sln);
    println!("solmate-ln: {}", sln);

    assert!((sln - rln).abs() < *EPSILON);

    let mut rng = thread_rng();
    for _ in 0..FUZZ_RUNS {
        let num: I256 = gen_nonzero_signed_wad(&mut rng);
        println!("Generated: {}", num);

        let sln = fuzz.ln(to_wad(num)).call().await?;
        let sln = wad_to_float(sln);
        println!("solmate-ln: {}", sln);

        let f = Float::with_val(
            F_PREC,
            Float::parse(num.to_string()).unwrap()
        );
        let rln = f.ln();
        println!("rust-ln: {}", rln);

        assert!((sln - rln).abs() < *EPSILON);
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    // instantiate our wallet & anvil
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // 3. connect to the network
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    // 4. instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Added a random constructor parameter
    let fuzz = Fuzz::deploy(client.clone(), ())?.send().await?;

    test_name(&fuzz).await?;
    test_add(&fuzz).await?;
    test_ln(&fuzz).await?;

    println!("Success!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rug_ln() {
        let f = Float::with_val(53, 1.5);
        assert_eq!(f.prec(), 53);
        assert_eq!(f, 1.5);
        let ln = f.ln();
        let expected = 0.4055_f64;
        assert!((ln - expected).abs() < 0.0001);
    }

    #[test]
    fn test_rug_ln_bigfloat() {
        use rug::Float;
        let f = Float::with_val(300, 1.5);
        assert_eq!(f.prec(), 300);
        assert_eq!(f, 1.5);
        let ln = f.ln();
        println!("{}", ln);
        let expected =
            Float::with_val(
                300,
                Float::parse("4.0546510810816438197801311546434913657199042346249419761401432414410067124891425126775242773e-1")
                    .unwrap()
            );
        assert!((ln - expected).abs() < *EPSILON);
    }

    #[test]
    fn test_tolerance() {
        println!("{}", *EPSILON);
    }

}
