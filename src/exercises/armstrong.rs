use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let pow = digits.len() as u32;

    let sum = digits
        .into_iter()
        .fold(0u64, |acc, x| acc + x.pow(pow) as u64);

    sum == num as u64
}
