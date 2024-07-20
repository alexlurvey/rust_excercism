use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reverse(input: &str) -> String {
    let mut s = String::new();

    for x in input.graphemes(true).rev() {
        // log_1(&tv(&x).unwrap());

        s.push_str(x)
    }

    s
}
