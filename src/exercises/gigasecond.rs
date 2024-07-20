use serde_wasm_bindgen::{from_value, to_value};
use time::{Duration, PrimitiveDateTime as DateTime};
use wasm_bindgen::prelude::*;

const GIGASECOND: Duration = Duration::seconds(1_000_000_000);

fn get_js_date_parts(date: &DateTime) -> Vec<i32> {
    vec![
        date.year(),
        date.month() as i32 - 1,
        date.day() as i32,
        date.hour() as i32,
        date.minute() as i32,
        date.second() as i32,
    ]
}

#[wasm_bindgen]
pub fn after(date: JsValue) -> JsValue {
    let datetime: DateTime = from_value(date).expect("expected date as a tuple of time components: [year, day_of_year, hours, minutes, seconds, milliseconds]");

    let plus_gigasecond = datetime
        .checked_add(GIGASECOND)
        .expect("provided date + gigasecond too large");

    let parts = get_js_date_parts(&plus_gigasecond);

    to_value(&parts).unwrap()
}
