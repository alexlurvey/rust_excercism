use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }

    let mut result: Vec<u64> = Vec::new();
    let mut remainder = n;

    while remainder > 1 {
        for x in 2..=remainder {
            if remainder % x == 0 {
                result.push(x);
                remainder = remainder / x;
                break;
            }
        }
    }

    result
}
