use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reverse(input: &str) -> String {
    let mut s = String::new();

    for x in input.graphemes(true).rev() {
        s.push_str(x)
    }

    s
}
