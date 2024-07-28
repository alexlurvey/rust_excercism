use wasm_bindgen::prelude::*;

fn square_of_sum(n: u32) -> u32 {
    (1..=n).fold(0u32, |acc, x| acc + x).pow(2)
}

fn sum_of_squares(n: u32) -> u32 {
    (1..=n).fold(0u32, |acc, x| acc + x.pow(2))
}

#[wasm_bindgen]
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
