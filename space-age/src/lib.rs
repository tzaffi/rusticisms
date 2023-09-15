const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / EARTH_YEAR_SECONDS,
        }
    }
}

pub trait Planet {
    const PERIOD_RATIO: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::PERIOD_RATIO
    }
}

macro_rules! planets {
    ($($planet:ident => $p_ratio:expr),* $(,)?) => {
        $(
            pub struct $planet;

            impl Planet for $planet {
                const PERIOD_RATIO: f64 = $p_ratio;
            }
        )*
    };
}

planets! {
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.79132,
}
