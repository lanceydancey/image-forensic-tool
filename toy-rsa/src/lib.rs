use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    let key: (u32, u32) = loop {
        let p: u32 = rsa_prime();
        let q: u32 = rsa_prime();

        let lcm_output = lcm((p as u64) - 1, (q as u64) - 1);

        if EXP < lcm_output && gcd(EXP, lcm_output) == 1 {
            break (p, q);
        }
    };
    println!("the key is: {} , {}", key.0, key.1);
    key
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}
/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d = modinverse(EXP, lcm((key.0 as u64) - 1, (key.1 as u64)) - 1);
    u32::try_from(msg).unwrap().pow(u32::try_from(d).unwrap()) % (key.0 * key.1)
}

fn main() {
    genkey();
}