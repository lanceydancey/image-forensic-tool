//! Command-line modular exponentation tool
//!
//! Lance Miller 2023

use std::str::FromStr;
use std::env;



fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() < 3 {
        error();
    }
    let x = numbers[0];

    let y = numbers[1];

    let m = numbers[2];

    modexp(x, y, m);
}

///Calculate the exponential modulo and print/return it
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    let mut base: u128 = u128::from(x);

    let mut exp: u128 = u128::from(y);

    let md: u128 = u128::from(m);

    if md == 1 {
        return 0;
    }

    let mut z: u128 = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            z = (z*base) % md;
        }
        exp /= 2;
        base =  base.pow(2)% md;

    }
    println!("The result is {} ", z);
    u64::try_from(z).unwrap()
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

///test for u64 overflow and other things
#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
}



