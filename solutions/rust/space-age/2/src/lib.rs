const SECONDS_PER_YEAR: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration {
            years: seconds as f64 / SECONDS_PER_YEAR,
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.years / Self::ORBITAL_PERIOD
    }
}

macro_rules! define_planet {
    ($name:ident, $period:expr) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

define_planet!(Mercury, 0.2408467);
define_planet!(Venus, 0.6151972);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.8808158);
define_planet!(Jupiter, 11.862615);
define_planet!(Saturn, 29.447498);
define_planet!(Uranus, 84.016846);
define_planet!(Neptune, 164.79132);
