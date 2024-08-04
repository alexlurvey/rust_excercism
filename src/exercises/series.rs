use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() || len == 0 {
        return vec![];
    }

    if len == digits.len() {
        return vec![String::from(digits)];
    }

    let mut result: Vec<String> = Vec::new();
    let upper = digits.len() - len + 1;

    for i in 0..upper {
        result.push(String::from(&digits[i..(i + len)]));
    }

    result
}

#[wasm_bindgen(js_name = "series")]
pub fn series_wasm(digits: &str, len: usize) -> JsValue {
    to_value(&series(digits, len)).unwrap()
}
