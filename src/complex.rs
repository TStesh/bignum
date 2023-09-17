/// Module for a complex number calculations
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy)]
pub struct Complex(pub f64, pub f64);

impl Complex {
    // zero
    pub fn zero() -> Self {
        Self(0., 0.)
    }
    // one
    pub fn one() -> Self {
        Self(1., 0.)
    }
    // conjugate
    pub fn conj(self) -> Self {
        Self(self.0, -self.1)
    }
    // |z|^2
    pub fn norm_sqr(self) -> f64 { self.0 * self.0 + self.1 * self.1 }
    // |z|
    pub fn module(self) -> f64 { self.norm_sqr().sqrt() }
    // standard form -> exponential form
    pub fn to_exp(self) -> Self {
        let rad = self.module();
        let phi = (self.1 / self.0).atan();
        Self(rad, phi)
    }
    // exponential form -> standard form
    pub fn from_exp(self) -> Self {
        Self(self.0 * self.1.cos(), self.0 * self.1.sin())
    }
}

//------------------------------------------------------------------------------------------------
// Функции сложения
impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; self.1 += rhs.1; }
}

impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}

impl AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) { self.0 += rhs; }
}

//------------------------------------------------------------------------------------------------
// Функции вычитания
impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; self.1 -= rhs.1; }
}

impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}

impl SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) { self.0 -= rhs; }
}

//------------------------------------------------------------------------------------------------
// Функции сложения
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.0 * rhs.0;
        let y = self.1 * rhs.1;
        let z = (self.0 + self.1) * (rhs.0 + rhs.1);
        Self(x - y, z - x - y)
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = (*self) * rhs;
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1)
    }
}

//------------------------------------------------------------------------------------------------
// Функции деления
impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        let x = rhs.0 * rhs.0 + rhs.1 * rhs.1;
        let y = self * rhs.conj();
        Self(y.0 / x, y.1 / x)
    }
}

impl Div<f64> for Complex {
    type Output = Complex;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self) / rhs;
    }
}

impl DivAssign<f64> for Complex {
    fn div_assign(&mut self, rhs: f64) {
        (*self).0 /= rhs;
        (*self).1 /= rhs;
    }
}

impl PartialEq<Self> for Complex {
    fn eq(&self, other: &Self) -> bool {
        let eps = 10f64.powi(-10);
        ((*self).0 - (*other).0).abs() < eps && ((*self).1 - (*other).1).abs() < eps
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("r", &self.0)
            .field("i", &self.1)
            .finish()
    }
}

//------------------------------------------------------------------------------------------------
// Функция отображения числа
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.1 > 0. {
            write!(f, "{:.5}+{:.5}j", self.0, self.1)
        } else if self.1 < 0. {
            write!(f, "{:.5}-{:.5}j", self.0, self.1.abs())
        } else {
            write!(f, "{:.5}", self.0)
        }
    }
}