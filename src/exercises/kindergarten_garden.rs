use wasm_bindgen::prelude::*;

const STUDENTS: [&'static str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows: Vec<&str> = diagram.split('\n').collect();
    let idx = STUDENTS.iter().position(|x| *x == student);

    if idx == None {
        return vec![];
    }

    let pos = idx.unwrap() * 2;

    let letters = vec![
        &rows[0][pos..=pos],
        &rows[0][pos + 1..=pos + 1],
        &rows[1][pos..=pos],
        &rows[1][pos + 1..=pos + 1],
    ];

    letters
        .iter()
        .map(|letter| {
            if *letter == "V" {
                return "violets";
            }

            if *letter == "C" {
                return "clover";
            }

            if *letter == "R" {
                return "radishes";
            }

            if *letter == "G" {
                return "grass";
            }

            return "";
        })
        .collect()
}

#[wasm_bindgen(js_name = "plants")]
pub fn plants_wasm(diagram: &str, student: &str) -> Vec<String> {
    plants(diagram, student)
        .iter()
        .map(|x| String::from(*x))
        .collect()
}
