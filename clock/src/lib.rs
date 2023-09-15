use std::fmt;

const MINUTES_IN_A_DAY: i32 = 24 * 60;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (60 * hours + minutes).rem_euclid(MINUTES_IN_A_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hours, minutes) = (self.minutes / 60, self.minutes % 60);

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
