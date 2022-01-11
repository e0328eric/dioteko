use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::ffi;

// Vector Implementations
impl_vec!(Vector2 | x, y);
impl_vec!(Vector3 | x, y, z);
impl_vec!(Vector4 | x, y, z, w);

// Quaternion Implementations
pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

impl From<ffi::Quaternion> for Quaternion {
    fn from(qua: ffi::Quaternion) -> Self {
        Self(qua.x, qua.y, qua.z, qua.w)
    }
}

impl From<Quaternion> for ffi::Quaternion {
    fn from(qua: Quaternion) -> ffi::Quaternion {
        ffi::Quaternion {
            x: qua.0,
            y: qua.1,
            z: qua.2,
            w: qua.3,
        }
    }
}

impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1 - self.2 * rhs.2 - self.3 * rhs.3,
            self.0 * rhs.1 + self.1 * rhs.0 + self.2 * rhs.3 - self.3 * rhs.2,
            self.0 * rhs.2 - self.1 * rhs.3 + self.2 * rhs.0 + self.3 * rhs.1,
            self.0 * rhs.3 + self.1 * rhs.2 - self.2 * rhs.1 + self.3 * rhs.0,
        )
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion(self * rhs.0, self * rhs.1, self * rhs.2, self * rhs.3)
    }
}

impl Div for Quaternion {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<Quaternion> for f32 {
    type Output = Quaternion;
    fn div(self, rhs: Quaternion) -> Self::Output {
        (self / (rhs.0 * rhs.0 + rhs.1 * rhs.1 + rhs.2 * rhs.2 + rhs.3 * rhs.3))
            * Quaternion(rhs.0, -rhs.1, -rhs.2, -rhs.3)
    }
}

// TODO: Implement Matrix
