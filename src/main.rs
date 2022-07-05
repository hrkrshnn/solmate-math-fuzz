mod rng;
mod wadmath;
mod float;

use rng::{gen_nonzero_signed_wad, gen_wad_for_exp};
use wadmath::{to_wad, wad_to_float};
use float::{F_PREC, EPSILON, is_within_tolerance_relative};

use ethers::prelude::*;

use ethers::{utils::Anvil};

use eyre::Result;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use tokio;
use rand::{thread_rng};
use rug::Float;
use clap::Parser;

#[macro_use]
extern crate lazy_static;

abigen!(Fuzz, "./out/Fuzz.sol/Fuzz.json");

type FuzzType = Fuzz<SignerMiddleware<Provider<Http>, LocalWallet>>;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Number of fuzz runs
    #[clap(
        help = "The number of fuzz runs",
        long = "fuzz-runs",
        default_value = "1000",
    )]
    fuzz_runs: u64,
    #[clap(
        help = "Fuzz wadln",
        long = "ln",
        required = false,
        takes_value = false,
    )]
    fuzz_ln: bool,
    #[clap(
        help = "Fuzz wadxp",
        long = "exp",
        required = false,
        takes_value = false,
    )]
    fuzz_exp: bool,
    #[clap(
        help = "Relative tolerance",
        long = "reltol",
        default_value = "0.001",
    )]
    reltol: f64
}

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

async fn test_ln(fuzz: &FuzzType, runs: u64) -> Result<()> {
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
    for _ in 0..runs {
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

async fn test_exp(fuzz: &FuzzType, runs: u64, reltol: f64) -> Result<()> {
    // Testing for exp(0)
    let exp = fuzz.exp(to_wad(0.into())).call().await?;
    assert_eq!(exp, 10u128.pow(18).into());

    let f = Float::with_val(F_PREC, 42);
    let rexp = f.exp();
    println!("rust-exp: {}", rexp);
    let sexp = fuzz.exp(to_wad(42.into())).call().await?;
    let sexp = wad_to_float(sexp);
    println!("solmate-exp: {}", sexp);

    let mut rng = thread_rng();
    for _ in 0..runs {
        let num: I256 = gen_wad_for_exp(&mut rng);
        println!("Generated: {}", num);

        let sexp = fuzz.exp(num).call().await?;
        let sexp = wad_to_float(sexp);
        println!("solmate-exp : {}", sexp);

        let num = wad_to_float(num);
        let f = Float::with_val(
            F_PREC,
            num
        );
        let rexp = f.exp();
        println!("rust-rug-exp: {}", rexp);

        assert!(is_within_tolerance_relative(rexp, sexp, reltol))
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    // instantiate our wallet & anvil
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // 3. connect to the network
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(100u64));

    // 4. instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Added a random constructor parameter
    let fuzz = Fuzz::deploy(client.clone(), ())?.send().await?;

    test_name(&fuzz).await?;
    test_add(&fuzz).await?;

    if args.fuzz_exp {
        test_exp(&fuzz, args.fuzz_runs, args.reltol).await?;
    }
    if args.fuzz_ln {
        test_ln(&fuzz, args.fuzz_runs).await?;
    }

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
