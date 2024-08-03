use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(list: &[T], candidate: &[T]) -> bool {
    if candidate.len() > list.len() {
        return false;
    }

    let start_indexes: Vec<usize> =
        list.into_iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, x)| {
                if x == &candidate[0] {
                    acc.push(i);
                }
                acc
            });

    if start_indexes.len() > 0 {
        if start_indexes.into_iter().any(|index| {
            return candidate
                .into_iter()
                .enumerate()
                .all(|(i, x)| i + index < list.len() && x == &list[i + index]);
        }) {
            return true;
        }
    }

    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == 0 && second_list.len() == 0 {
        return Comparison::Equal;
    }

    if first_list.len() == 0 {
        return Comparison::Sublist;
    }

    if second_list.len() == 0 {
        return Comparison::Superlist;
    }

    if first_list.len() == second_list.len() {
        let eq = first_list
            .into_iter()
            .enumerate()
            .all(|(i, x)| x == &second_list[i]);

        if eq {
            return Comparison::Equal;
        }

        return Comparison::Unequal;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Sublist;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

#[wasm_bindgen(js_name = "sublist")]
pub fn _sublist(first_list: JsValue, second_list: JsValue) -> Comparison {
    let one: Vec<usize> = from_value(first_list).unwrap();
    let two: Vec<usize> = from_value(second_list).unwrap();

    sublist(&one, &two)
}
