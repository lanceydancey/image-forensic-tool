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

    modexp(2, 20, 17);
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
        y = y/2;
        x =  x.pow(2)% m;

    }
    println!("The result is {} ", z);
    z
}



