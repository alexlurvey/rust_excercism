use wasm_bindgen::prelude::*;

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.len() == 0 {
        return "Fine. Be that way!";
    }

    let letters: Vec<char> = trimmed
        .chars()
        .filter(|x| matches!(x, 'A'..='Z' | 'a'..='z'))
        .collect();

    let is_question = trimmed.ends_with("?");
    let is_yelling = letters.len() > 0
        && letters
            .iter()
            .all(|x| !x.is_whitespace() && x.is_uppercase());

    if is_question && is_yelling {
        return "Calm down, I know what I'm doing!";
    }

    if is_question {
        return "Sure.";
    }

    if is_yelling {
        return "Whoa, chill out!";
    }

    "Whatever."
}

#[wasm_bindgen(js_name = "bob")]
pub fn reply_wasm(message: String) -> String {
    String::from(reply(message.as_str()))
}
