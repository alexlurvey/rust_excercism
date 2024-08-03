use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return vec![];
    }

    if minefield[0].is_empty() {
        return vec![String::from("")];
    }

    let mut result: Vec<String> = Vec::new();
    let mf = minefield
        .into_iter()
        .map(|x| x.as_bytes())
        .collect::<Vec<&[u8]>>();
    let rows = mf.len() - 1;
    let columns = mf[0].len() - 1;

    for row in 0..=rows {
        let mut row_str: String = String::new();

        for column in 0..=columns {
            if mf[row][column] == b'*' {
                row_str.push('*');
                continue;
            }

            let mut count: u8 = 0;
            let mut count_mine = |r: usize, c: usize| {
                if mf[r][c] == b'*' {
                    count += 1;
                }
            };

            // top left
            if row > 0 && column > 0 {
                count_mine(row - 1, column - 1);
            }

            // top middle
            if row > 0 {
                count_mine(row - 1, column);
            }

            // top right
            if row > 0 && column < columns {
                count_mine(row - 1, column + 1);
            }

            // right
            if column < columns {
                count_mine(row, column + 1);
            }

            // bottom right
            if row < rows && column < columns {
                count_mine(row + 1, column + 1);
            }

            // bottom middle
            if row < rows {
                count_mine(row + 1, column);
            }

            // bottom left
            if row < rows && column > 0 {
                count_mine(row + 1, column - 1);
            }

            // left
            if column > 0 {
                count_mine(row, column - 1);
            }

            if count == 0 {
                row_str.push(' ');
            } else {
                row_str.push_str(format!("{count}").as_str());
            }
        }

        result.push(row_str);
    }

    result
}

#[wasm_bindgen(js_name = "annotate")]
pub fn annotate_wasm(minefield: JsValue) -> JsValue {
    let mf: Vec<String> = from_value(minefield).unwrap();
    let mf: Vec<&str> = mf.iter().map(AsRef::as_ref).collect();
    let result = annotate(&mf);

    to_value(&result).unwrap()
}
