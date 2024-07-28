use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "luhn")]
pub fn is_valid(code: &str) -> bool {
    let trimmed = code.split("").fold(Vec::new(), |mut acc, s| {
        if s != " " && s != "" {
            acc.push(s);
        }

        acc
    });

    if trimmed.len() < 2 {
        return false;
    }

    let parsed = trimmed.iter().fold(Vec::new(), |mut acc, s| {
        let int = s.parse::<u8>();

        if let Ok(x) = int {
            acc.push(x);
        }

        acc
    });

    if trimmed.len() != parsed.len() {
        return false;
    }

    let sum = parsed
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 1 {
                let xx = x * 2;
                let xx = if xx > 9 { xx - 9 } else { xx };
                return xx;
            }

            *x
        })
        .fold(0u16, |acc, x| acc + x as u16);

    sum % 10 == 0
}
