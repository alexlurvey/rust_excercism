use wasm_bindgen::prelude::*;

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

#[wasm_bindgen]
pub struct Mercury;
#[wasm_bindgen]
pub struct Venus;
#[wasm_bindgen]
pub struct Earth;
#[wasm_bindgen]
pub struct Mars;
#[wasm_bindgen]
pub struct Jupiter;
#[wasm_bindgen]
pub struct Saturn;
#[wasm_bindgen]
pub struct Uranus;
#[wasm_bindgen]
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (0.2408467 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (0.61519726 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / EARTH_YEAR_SECONDS
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (1.8808158 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (11.862615 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (29.447498 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (84.016846 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (164.79132 * EARTH_YEAR_SECONDS)
    }
}

#[wasm_bindgen(js_name = "Planet")]
pub enum PlanetEnum {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

#[wasm_bindgen]
pub fn years_during(s: u64, planet: PlanetEnum) -> f64 {
    let d = Duration::from(s);

    match planet {
        PlanetEnum::Mercury => Mercury::years_during(&d),
        PlanetEnum::Venus => Venus::years_during(&d),
        PlanetEnum::Earth => Earth::years_during(&d),
        PlanetEnum::Mars => Mars::years_during(&d),
        PlanetEnum::Jupiter => Jupiter::years_during(&d),
        PlanetEnum::Saturn => Saturn::years_during(&d),
        PlanetEnum::Uranus => Uranus::years_during(&d),
        PlanetEnum::Neptune => Neptune::years_during(&d),
    }
}
