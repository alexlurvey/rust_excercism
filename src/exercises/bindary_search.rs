use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    if array.len() == 1 {
        return if array[0] == key { Some(0) } else { None };
    }

    let mut index = array.len() / 2;
    let mut real_index = index;
    let mut slice = array;

    while slice.len() > 1 {
        if slice[index] == key {
            return Some(real_index);
        }

        if slice[index] > key {
            slice = &slice[0..index];
            index = slice.len() / 2;
            real_index -= index;

            if slice.len() % 2 != 0 {
                real_index -= 1;
            }

            if slice.len() == 1 && slice[0] == key {
                return Some(real_index);
            }
        } else {
            slice = &slice[(index + 1)..];
            index = slice.len() / 2;
            real_index += index + 1;

            if slice.len() == 1 && slice[0] == key {
                return Some(real_index);
            }
        }
    }

    None
}
