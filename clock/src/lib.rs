// use std::fmt;

// #[derive(Debug, PartialEq)]
// pub struct Clock {
//     hours: i32,
//     minutes: i32,
// }

// fn convert_minutes_to_time(time_in_minutes: i32) -> Clock {
//     let mut actual_hours = 0;

//     let mut actual_minutes = time_in_minutes % 60;
//     if actual_minutes < 0 {
//         actual_minutes += 60;
//         actual_hours -= 1;
//     }

//     actual_hours = (time_in_minutes / 60) % 24 + actual_hours;
//     if actual_hours < 0 {
//         actual_hours += 24;
//     }

//     Clock {
//         hours: actual_hours,
//         minutes: actual_minutes,
//     }
// }

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         let time_in_minutes = hours * 60 + minutes;
//         convert_minutes_to_time(time_in_minutes)
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         let time_in_minutes = self.hours * 60 + self.minutes + minutes;
//         convert_minutes_to_time(time_in_minutes)
//     }
// }

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:02}:{:02}", self.hours, self.minutes)
//     }
// }

// // Solution 2
// use std::fmt;

// #[derive(Debug, PartialEq)]
// pub struct Clock {
//     minutes: i32,
// }

// fn convert_minutes_to_time(time_in_minutes: i32) -> [i32; 2] {
//     let mut actual_hours = 0;

//     let mut actual_minutes = time_in_minutes % 60;
//     if actual_minutes < 0 {
//         actual_minutes += 60;
//         actual_hours -= 1;
//     }

//     actual_hours = (time_in_minutes / 60) % 24 + actual_hours;
//     if actual_hours < 0 {
//         actual_hours += 24;
//     }

//     [actual_hours, actual_minutes]
// }

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         let time_in_minutes = hours * 60 + minutes;
//         Clock {
//             minutes: time_in_minutes,
//         }
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         Clock {
//             minutes: self.minutes + minutes,
//         }
//     }
// }

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let [hours, minutes] = convert_minutes_to_time(self.minutes);
//         write!(f, "{:02}:{:02}", hours, minutes)
//     }
// }

// Solution 3
use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: Duration,
    minutes: Duration,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: Duration::hours(hours as i64),
            minutes: Duration::minutes(minutes as i64),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + Duration::minutes(minutes as i64),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = NaiveTime::from_hms(0, 0, 0) + self.hours + self.minutes;
        write!(f, "{}", time.format("%H:%M"))
    }
}
