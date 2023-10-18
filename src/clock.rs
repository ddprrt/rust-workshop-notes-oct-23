pub struct Clock {
    hours: u32,
    minutes: u32,
}

impl Clock {
    pub fn new(hours: u32, minutes: u32) -> Self {
        Clock { hours, minutes }.normalize()
    }

    fn normalize(self) -> Self {
        let mut hours = self.hours;
        let mut minutes = self.minutes;
        if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }
        if hours >= 24 {
            hours %= 24;
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(self, minutes: u32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
        .normalize()
    }
}

impl std::ops::Add for Clock {
    type Output = Clock;

    fn add(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}

impl std::ops::Add<u32> for Clock {
    type Output = Clock;

    fn add(self, rhs: u32) -> Self::Output {
        self.add_minutes(rhs)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
