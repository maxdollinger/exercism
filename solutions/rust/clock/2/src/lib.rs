const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = (hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY);
        Clock { minutes: total }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hours = self.minutes / MINUTES_PER_HOUR;
        let mins = self.minutes % MINUTES_PER_HOUR;
        write!(f, "{:02}:{:02}", hours, mins)
    }
}
