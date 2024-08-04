use rand::Rng;
use wasm_bindgen::prelude::*;

// right-to-left binary method: https://en.wikipedia.org/wiki/Modular_exponentiation
fn fast_diffie_hellman(base: u64, exp: u64, modulo: u64) -> u64 {
    let mut r = 1u64;
    let mut e = exp;
    let mut b = base;

    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % modulo;
        }
        e = e >> 1;
        b = b.pow(2) % modulo;
    }

    return r;
}

#[wasm_bindgen]
pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

#[wasm_bindgen]
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    if a > u64::from(u32::MAX) {
        return fast_diffie_hellman(g, a, p);
    }

    g.pow(a as u32) % p
}

#[wasm_bindgen]
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    if a > u64::from(u32::MAX) {
        return fast_diffie_hellman(b_pub, a, p);
    }

    b_pub.pow(a as u32) % p
}
