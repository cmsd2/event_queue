use std::fmt;
use std::ops;
use std::cmp;

#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Time {
    pub ticks: u32,
    pub micro_ticks: u32
}

impl Default for Time {
    fn default() -> Self {
        Time::new(0, 0)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let precision = cmp::max(0, cmp::min(6, f.precision().unwrap_or(6)));
        if precision == 0 {
            write!(f, "{}", self.ticks)
        } else {
            let mut d = 6 - precision;
            let mut micro_ticks = self.micro_ticks;
            while d > 0 {
                micro_ticks /= 10;
                d -= 1;
            }
            write!(f, "{}.{:0width$}", self.ticks, micro_ticks, width=precision)
        }
    }
}

impl Time {
    pub fn new(ticks: u32, micro_ticks: u32) -> Self {
        let mut time = Time { ticks, micro_ticks };
        time.normalise();
        time
    }

    pub fn normalise(&mut self) {
        self.ticks += self.micro_ticks / 1000000;
        self.micro_ticks = self.micro_ticks % 1000000;
    }
}

impl ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Self::Output {
        Time::new(self.ticks + rhs.ticks, self.micro_ticks + rhs.micro_ticks)
    }
}

impl ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Time) {
        self.ticks += rhs.ticks;
        self.micro_ticks += rhs.micro_ticks;
        self.normalise();
    }
}

impl ops::Add<u32> for Time {
    type Output = Time;

    fn add(self, rhs: u32) -> Self::Output {
        Time::new(self.ticks + rhs, self.micro_ticks)
    }
}

impl ops::AddAssign<u32> for Time {
    fn add_assign(&mut self, rhs: u32) {
        self.ticks += rhs;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_display_time() {
        let t = Time::new(1, 1000);
        
        assert_eq!(format!("{}", t), "1.001000");
        assert_eq!(format!("{:.0}", t), "1");
        assert_eq!(format!("{:.3}", t), "1.001");
        assert_eq!(format!("{:.6}", t), "1.001000");
    }
}