use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn brackets_are_balanced(string: &str) -> bool {
    let map: HashMap<char, char> = HashMap::from_iter([('{', '}'), ('(', ')'), ('[', ']')]);

    let brackets: Vec<char> = string
        .chars()
        .into_iter()
        .filter(|c| map.contains_key(c) || map.values().into_iter().any(|x| *x == *c))
        .collect();

    if brackets.len() % 2 == 1 {
        return false;
    }

    let mut stack: Vec<char> = Vec::new();

    for c in brackets {
        if map.contains_key(&c) {
            stack.push(*map.get(&c).unwrap());
            continue;
        }

        if stack.len() == 0 {
            return false;
        }

        let last = *stack.last().unwrap();

        if c == '}' && last == '}' {
            stack.pop();
        } else if c == ']' && last == ']' {
            stack.pop();
        } else if c == ')' && last == ')' {
            stack.pop();
        } else {
            return false;
        }
    }

    stack.len() == 0
}
