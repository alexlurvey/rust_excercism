use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0u32;
    let mut num = display_value;

    while num > 0 {
        count += num & 1;
        num = num >> 1;
    }

    count as usize
}
