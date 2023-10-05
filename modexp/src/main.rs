use std::str::FromStr;
use std::env;



fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() < 3 {
        eprintln!("Usage: modexp <x> <y> <m>");
        std::process::exit(1);
    }
    let x = numbers[0];

    let y = numbers[1];

    let m = numbers[2];

    modexp(x, y, m);
}

fn modexp(mut x: u64, mut y: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut z: u64 = 1;

    while y > 0 {
        if y % 2 == 1 {
            z = (z*x) % m;
        }
        y /= 2;
        x =  x.pow(2)% m;

    }
    println!("The result is {} ", z);
    z
}

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



