// // The code below is a stub. Just enough to satisfy the compiler.
// // In order to pass the tests you can add-to or change any of this code.

// #[derive(Debug)]
// pub struct Duration {
//     seconds: f64,
// }

// impl From<u64> for Duration {
//     fn from(s: u64) -> Self {
//         Duration { seconds: s as f64 }
//     }
// }

// // Convert seconds to earth years.
// fn sec_to_earth_yr(d: &Duration) -> f64 {
//     let sec_to_min = 1.0 / 60.0;
//     let min_to_hr = 1.0 / 60.0;
//     let hr_to_day = 1.0 / 24.0;
//     let day_to_yr = 1.0 / 365.25;
//     let yr_to_earth = d.seconds * sec_to_min * min_to_hr * hr_to_day * day_to_yr;
//     yr_to_earth
// }

// pub trait Planet {
//     fn years_during(d: &Duration) -> f64;
// }

// pub struct Mercury;
// pub struct Venus;
// pub struct Earth;
// pub struct Mars;
// pub struct Jupiter;
// pub struct Saturn;
// pub struct Uranus;
// pub struct Neptune;

// impl Planet for Mercury {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 0.2408467
//     }
// }
// impl Planet for Venus {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 0.61519726
//     }
// }
// impl Planet for Earth {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d)
//     }
// }
// impl Planet for Mars {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 1.8808158
//     }
// }
// impl Planet for Jupiter {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 11.862615
//     }
// }
// impl Planet for Saturn {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 29.447498
//     }
// }
// impl Planet for Uranus {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 84.016846
//     }
// }
// impl Planet for Neptune {
//     fn years_during(d: &Duration) -> f64 {
//         sec_to_earth_yr(d) / 164.79132
//     }
// }

pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / (31557600 as f64))
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::period()
    }
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn period() -> f64 {
                $p
            }
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
