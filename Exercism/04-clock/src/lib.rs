use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        Clock::new(self.hours, self.minutes).to_string()
            == Clock::new(other.hours, other.minutes).to_string()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h1 = hours;
        let mut m1 = minutes;

        if m1 >= 60 || m1 < 0 {
            if m1 < 0 {
                h1 += (m1 / 60) - 1;
                m1 = (m1 % 60) + 60;
            }

            h1 += m1 / 60;
            m1 = m1 % 60;
        }

        if h1 >= 24 || h1 < 0 {
            if h1 < 0 {
                h1 = (h1 % 24) + 24;
            }

            h1 = h1 % 24;
        }

        Clock {
            hours: h1,
            minutes: m1,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
