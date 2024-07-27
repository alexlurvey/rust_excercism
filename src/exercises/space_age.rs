use wasm_bindgen::prelude::*;

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

macro_rules! planet_impl {
    ($t:ty, $x:expr) => {
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / ($x * EARTH_YEAR_SECONDS)
            }
        }
    };
}

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

planet_impl!(Mercury, 0.2408467);
planet_impl!(Venus, 0.61519726);
planet_impl!(Earth, 1.0);
planet_impl!(Mars, 1.8808158);
planet_impl!(Jupiter, 11.862615);
planet_impl!(Saturn, 29.447498);
planet_impl!(Uranus, 84.016846);
planet_impl!(Neptune, 164.79132);

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
