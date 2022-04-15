use std::fmt;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock{
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes : minutes.rem_euclid(60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::new(self.hours, self.minutes+minutes)
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq<Clock> for Clock{
    fn eq(&self, other: &Clock) -> bool {
        return self.hours == other.hours && self.minutes == other.minutes 
    }
}

impl Add<Clock> for Clock {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.hours + other.hours, self.minutes + other.minutes)
    }
}

impl Add<i32> for Clock {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        Self::new(self.hours,self.minutes + other)
    }
}

impl Sub<Clock> for Clock {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.hours - other.hours, self.minutes - other.minutes)
    }
}

impl Sub<i32> for Clock {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        Self::new(self.hours,self.minutes - other)
    }
}


