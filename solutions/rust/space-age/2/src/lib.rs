const SECONDS_IN_EARTH_YEAR: u64 = 31_557_600;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    fn orbital_period_in_earth_years() -> f64;
    
    fn years_during(d: &Duration) -> f64 {
        let earth_year = d.seconds as f64 / SECONDS_IN_EARTH_YEAR as f64;
        earth_year / Self::orbital_period_in_earth_years()
    }
}

macro_rules! planet {
    ($($t:ident: $v:expr),+) => {
        $(pub struct $t;)+
        $(impl Planet for $t {
            fn orbital_period_in_earth_years() -> f64 {
                $v
            }
        })+
    };
}

planet! {
    Mercury: 0.2408467,
    Venus: 0.61519726,
    Earth: 1.0,
    Mars: 1.8808158,
    Jupiter: 11.862615,
    Saturn: 29.447498,
    Uranus: 84.016846,
    Neptune: 164.79132
}