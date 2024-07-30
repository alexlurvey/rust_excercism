use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    if list.len() == 1 {
        return format!("And all for the want of a {}", list[0]);
    }

    let mut result: Vec<String> = Vec::new();

    for x in 1..list.len() {
        result.push(format!(
            "For want of a {} the {} was lost.",
            list[x - 1],
            list[x]
        ));
    }

    result.push(format!("And all for the want of a {}.", list[0]));

    result.join("\n")
}

#[wasm_bindgen(js_name = "build_proverb")]
pub fn build_proverb_wasm(list: JsValue) -> String {
    let owned: Vec<String> = from_value(list).unwrap();
    let strs: Vec<&str> = owned.iter().map(AsRef::as_ref).collect();

    build_proverb(&strs)
}
