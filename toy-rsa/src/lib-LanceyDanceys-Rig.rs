use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    let key: (u32, u32) = loop {
        let p: u32 = rsa_prime();
        let q: u32 = rsa_prime();

        if EXP < lcm(p as u64, q as u64) && gcd(EXP, lcm(p as u64, q as u64)) == 1 {
            break (p, q);
        }
    };
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
    let d: u64 = modinverse(EXP, lcm((key.0 as u64) - 1, (key.1 as u64) - 1));

    let d_msg: u64 = modexp(msg, d, (key.0 * key.1) as u64);

    u32::try_from(d_msg).unwrap()
}

///test genkey for returning prime number
#[test]
fn test_genkey() {
    let (p, q) = genkey();
    assert!(EXP < lcm(p as u64, q as u64));
    assert_eq!(1, gcd(EXP, lcm(p as u64, q as u64)));
}

//test that encrypted message matches test
#[test]
fn test_encrypt() {
    let pub_key: u64 = 0xde9c5816141c8ba9;
    let msg: u32 = 0x12345f;
    let encrypted_message: u64 = 0x6418280e0c4d7675;
    assert_eq!(encrypted_message, encrypt(pub_key, msg));
}

//test the decryption
#[test]
fn test_decrypt() {
    let private_key: (u32, u32) = (0xed23e6cd, 0xf050a04d);
    let encrypted_message: u64 = 0x6418280e0c4d7675;
    let decrypted_message: u32 = 0x12345f;
    assert_eq!(decrypted_message, decrypt(private_key, encrypted_message));
}
