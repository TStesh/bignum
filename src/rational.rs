use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

enum Sign {
    Plus(i64, i64),
    Minus(i64, i64),
    Null
}

pub struct Ratio {
    numerator: i64,
    denominator: i64,
}

impl Ratio {
    // new rational number
    pub fn new(n: i64, d: i64) -> Self {
        if d != 0 {
            let g = if n != 0 {
                gcd(n.abs() as u64, d.abs() as u64) as i64
            } else {
                1
            };
            Ratio {
                numerator: n / g,
                denominator: d / g
            }
        } else {
            panic!("division by zero");
        }
    }
    // new reciprocal number
    pub fn reciproc(n: i64) -> Self {
        if n != 0 {
            Ratio {
                numerator: 1i64,
                denominator: n
            }
        } else {
            panic!("division by zero");
        }
    }
    // sign of the rational number
    fn sign(&self) -> Sign {
        if self.numerator == 0 { return Sign::Null }
        if (self.numerator > 0 && self.denominator > 0) ||
            (self.numerator < 0 && self.denominator < 0) {
            Sign::Plus(self.numerator.abs(), self.denominator.abs())
        } else {
            Sign::Minus(self.numerator.abs(), self.denominator.abs())
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.sign() {
            Sign::Null => write!(f, "0"),
            Sign::Plus(x, y) => if y != 1 {
                write!(f, "({x}/{y})")
            } else {
                write!(f, "{x}")
            },
            Sign::Minus(x, y) => if y != 1 {
                write!(f, "-({x}/{y})")
            } else {
                write!(f, "-{x}")
            }
        }
    }
}

impl From<i64> for Ratio {
    fn from(x: i64) -> Self {
        Ratio {
            numerator: x,
            denominator: 1i64
        }
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 { return x }
    if x >= y {
        gcd(y, x % y)
    } else {
        gcd(x, y % x)
    }
}


impl Add for Ratio {
    type Output = Ratio;
    fn add(self, rhs: Self) -> Self::Output {
        // numer * that.denom + denom * that.numer, denom * that.denom
        Ratio::new(self.numerator * rhs.denominator + self.denominator * rhs.numerator,
                   self.denominator * rhs.denominator)
    }
}

impl AddAssign for Ratio {
    fn add_assign(&mut self, rhs: Self) {
        let x = Ratio::new(self.numerator, self.denominator) + rhs;
        self.denominator = x.denominator;
        self.numerator = x.numerator;
    }
}

impl Sub for Ratio {
    type Output = Ratio;
    fn sub(self, rhs: Self) -> Self::Output {
        // numer * that.denom - denom * that.numer, denom * that.denom
        Ratio::new(self.numerator * rhs.denominator - self.denominator * rhs.numerator,
                   self.denominator * rhs.denominator)
    }
}

impl SubAssign for Ratio {
    fn sub_assign(&mut self, rhs: Self) {
        let x = Ratio::new(self.numerator, self.denominator) - rhs;
        self.denominator = x.denominator;
        self.numerator = x.numerator;
    }
}

impl Mul for Ratio {
    type Output = Ratio;
    fn mul(self, rhs: Self) -> Self::Output {
        // numer * that.numer, denom * that.denom
        Ratio::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator)
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        let x = Ratio::new(self.numerator, self.denominator) * rhs;
        self.denominator = x.denominator;
        self.numerator = x.numerator;
    }
}

impl Div for Ratio {
    type Output = Ratio;
    fn div(self, rhs: Self) -> Self::Output {
        // numer * that.denom, denom * that.numer
        Ratio::new(self.numerator * rhs.denominator, self.denominator * rhs.numerator)
    }
}

impl DivAssign for Ratio {
    fn div_assign(&mut self, rhs: Self) {
        let x = Ratio::new(self.numerator, self.denominator) / rhs;
        self.denominator = x.denominator;
        self.numerator = x.numerator;
    }
}
