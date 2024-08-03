use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();

    for factor in factors.into_iter().filter(|&x| *x != 0) {
        set.extend(((*factor)..limit).step_by(*factor as usize))
    }

    set.into_iter().fold(0u32, |acc, x| acc + x)
}
