use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3d(pub f64, pub f64, pub f64);

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3d(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3d(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3d> for f64 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Self::Output {
        Vec3d(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f64> for Vec3d {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;
    
    fn div(self, rhs: f64) -> Self::Output {
        Vec3d(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}