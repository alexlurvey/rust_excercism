use serde_wasm_bindgen::{from_value, to_value};
use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let word_lowercase = word.to_lowercase();

    for maybe_anagram in possible_anagrams.iter() {
        let test_word = maybe_anagram.to_lowercase();

        if word_lowercase.len() != test_word.len() {
            continue;
        }

        if word_lowercase.as_str() == test_word.as_str() {
            continue;
        }

        let mut letter_counts = test_word
            .graphemes(false)
            .fold(HashMap::new(), |mut acc, x| {
                let counter = acc.entry(x).or_insert(0);
                *counter += 1;
                acc
            });

        for slice in word_lowercase.graphemes(false) {
            if let Some(count) = letter_counts.get(slice) {
                if *count > 0 {
                    letter_counts.insert(slice, *count - 1);
                }
            }
        }

        let all_letters_used = letter_counts.values().all(|x| *x == 0);

        if all_letters_used {
            result.insert(&maybe_anagram);
        }
    }

    result
}

#[wasm_bindgen(js_name = "anagram_for")]
pub fn _anagram_for(word: &str, possible_anagrams: JsValue) -> JsValue {
    let anagrams: Vec<String> = from_value(possible_anagrams).unwrap();
    let anagrams_strs: Vec<&str> = anagrams.iter().map(AsRef::as_ref).collect();

    let response = anagrams_for(word, &anagrams_strs);

    to_value(&response).unwrap()
}
