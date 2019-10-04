use std::fmt;
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let carry = minutes.div_euclid(60);
        Clock {
            hours: (hours + carry).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let carry = (self.minutes + minutes).div_euclid(60);
        Clock {
            hours: (self.hours + carry).rem_euclid(24),
            minutes: (self.minutes + minutes).rem_euclid(60),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
