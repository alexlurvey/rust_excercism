use std::fmt::Display;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

#[wasm_bindgen]
impl Clock {
    #[wasm_bindgen(constructor)]
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_from_minutes_value = if minutes.abs() > 59 { minutes / 60 } else { 0 };
        let total_hours = hours + hours_from_minutes_value;

        let mut minutes_result = if minutes.abs() > 59 {
            minutes % 60
        } else {
            minutes
        };

        let mut hours_result = if total_hours.abs() > 23 {
            total_hours % 24
        } else {
            total_hours
        };

        if minutes_result < 0 {
            hours_result = hours_result - 1;
            minutes_result = minutes_result + 60;
        }

        if hours_result < 0 {
            hours_result = hours_result + 24;
        }

        if hours_result == 24 {
            hours_result = 0;
        }

        Clock {
            hours: hours_result,
            minutes: minutes_result,
        }
    }

    #[wasm_bindgen]
    pub fn stringify(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen]
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hrs = if minutes.abs() > 59 { minutes / 60 } else { 0 };
        let mins = if minutes.abs() > 59 {
            minutes % 60
        } else {
            minutes
        };

        Clock::new(self.hours + hrs, self.minutes + mins)
    }

    #[wasm_bindgen(getter)]
    pub fn minutes(&self) -> i32 {
        self.minutes
    }

    #[wasm_bindgen(setter)]
    pub fn set_minutes(&mut self, value: i32) {
        self.minutes = value;
    }

    #[wasm_bindgen(getter)]
    pub fn hours(&self) -> i32 {
        self.hours
    }

    #[wasm_bindgen(setter)]
    pub fn set_hours(&mut self, value: i32) {
        self.hours = value;
    }
}
