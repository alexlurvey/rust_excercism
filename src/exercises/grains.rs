use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grains_square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    if s == 1 {
        return 1;
    }

    2u64.pow(s - 1)
}

#[wasm_bindgen]
pub fn grains_total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + grains_square(x))
}
