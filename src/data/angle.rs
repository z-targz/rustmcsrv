use std::ops;

pub const CONVERSION_FACTOR_TO_NETWORK: f64 = 256.0 / 360.0;
pub const CONVERSION_FACTOR_FROM_NETWORK: f64 = 360.0 / 256.0;

#[derive(Debug, Copy, Clone)]
pub struct Angle {
    degrees: f64,
}

impl Angle {
    pub fn new(degrees: f64) -> Self {
        Angle {degrees : degrees}
    }
    pub fn get_degrees(&self) -> f64 {
        self.degrees
    }
    pub fn set_degrees(&mut self, degrees_new: f64) {
        self.degrees = degrees_new % 360.0;
    }
}

impl ops::Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Angle { degrees : (self.degrees + rhs.degrees) % 360.0 }
    }
}

impl ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.degrees = (self.degrees + rhs.degrees) % 360.0;
    }
}

impl ops::Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Angle { degrees : (self.degrees - rhs.degrees) % 360.0 }
    }
}

impl ops::SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.degrees = (self.degrees - rhs.degrees) % 360.0;
    }
}

impl ops::Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Self::Output {
        Angle { degrees : (self.degrees * rhs) % 360.0 }
    }
}

impl ops::Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        Angle { degrees : (rhs.degrees * self) % 360.0 }
    }
}

impl ops::MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        self.degrees = (self.degrees * rhs) % 360.0;
    }
}

impl ops::Div<f64> for Angle {
    type Output = Angle;

    fn div(self, rhs: f64) -> Self::Output {
        Angle { degrees : (self.degrees / rhs) % 360.0 }
    }
}

impl ops::DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        self.degrees = (self.degrees / rhs) % 360.0;
    }
}