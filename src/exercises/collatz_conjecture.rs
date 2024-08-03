use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut next = n;
    let mut steps = 0u64;

    while next > 1 {
        if next % 2 == 0 {
            next = next / 2;
        } else {
            next = next * 3 + 1;
        }
        steps += 1;
    }

    Some(steps)
}
