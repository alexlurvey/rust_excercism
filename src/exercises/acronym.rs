use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-', '_'])
        .filter(|x| !x.is_empty() && *x != " " && *x != "-")
        .fold(String::new(), |mut acc, word| {
            acc.extend(
                word.chars()
                    .take(1)
                    .chain(
                        word.chars()
                            .skip_while(|c| c.is_uppercase())
                            .filter(|c| c.is_uppercase()),
                    )
                    .map(|c| c.to_ascii_uppercase()),
            );

            acc
        })
}
