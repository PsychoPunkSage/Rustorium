use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    return rand::thread_rng().gen_range(2..p);
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let p = BigUint::from(p);
    let g = BigUint::from(g);
    let a = BigUint::from(a);
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let b_pub = BigUint::from(b_pub);
    let p = BigUint::from(p);
    let a = BigUint::from(a);
    mod_pow(b_pub, a, p)
}

fn mod_pow(base: BigUint, exp: BigUint, modulus: BigUint) -> u64 {
    let mut result = BigUint::one();
    let mut base = base % &modulus;
    let mut exp = exp;

    while !exp.is_zero() {
        if &exp % 2_u32 == BigUint::one() {
            result = (result * &base) % &modulus;
        }
        exp /= 2_u32;
        base = (&base * &base) % &modulus;
    }

    result.try_into().unwrap()
}
