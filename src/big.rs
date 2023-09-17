use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::str::FromStr;
use crate::complex::Complex;
use crate::ft::RevCash;
use crate::oper::{add_vec, mul_vec, sqr};

// Положительные целые большие числа
// Самая младшая цифра числа в массиве идет первой (порядок big endian)
pub struct BigDecimal {
    pub digits: Vec<u8>
}

impl BigDecimal {
    pub fn zero() -> Self {
        Self { digits: vec![0u8] }
    }
    pub fn one() -> Self {
        Self { digits: vec![1u8] }
    }
    pub fn sqr(&self) -> Self { Self { digits: sqr(&self.digits) } }
}

impl Display for BigDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.digits.iter().rev().map(|d| write!(f, "{d}")).collect()
    }
}

// Преобразование BigDecimal::from("<string>")
// Если <string> = <x>*<y>, то считаем что <x> надо повторить <y> раз
impl From<&str> for BigDecimal {
    fn from(value: &str) -> Self {
        match value.find("*") {
            Some(index) => parse_str(value, index),
            None => Self { digits: to_vec(value) }
        }
    }
}

// Преобразование BigDecimal::from("<&[u8]>")
impl From<&[u8]> for BigDecimal {
    fn from(value: &[u8]) -> Self {
        Self { digits: value.to_vec() }
    }
}

// a + b
impl Add for BigDecimal {
    type Output = BigDecimal;
    fn add(self, rhs: Self) -> Self::Output {
        Self { digits: add_vec(&self.digits, &rhs.digits) }
    }
}

// a += b
impl AddAssign for BigDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.digits = add_vec(&self.digits, &rhs.digits);
    }
}

// a * b
impl Mul for BigDecimal {
    type Output = BigDecimal;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { digits: mul_vec(&self.digits, &rhs.digits) }
    }
}

impl MulAssign for BigDecimal {
    fn mul_assign(&mut self, rhs: Self) {
        self.digits = mul_vec(&self.digits, &rhs.digits);
    }
}

// Преобразовать строку в вектор
fn to_vec(s: &str) -> Vec<u8> {
    s.as_bytes()
        .into_iter()
        .rev().map(|x| *x - 48)
        .collect()
}

// Максимальная длина подстроки из 0
fn null_substring(s: &str) -> usize {
    let r: Vec<u8> = s.as_bytes()
        .iter()
        .rev()
        .take_while(|x| **x == 48)
        .map(|x| *x)
        .collect();
    r.len()
}

// Парсер строки со *
fn parse_str(s: &str, index: usize) -> BigDecimal {
    if index == 0 {
        panic!("Incorrect syntax before *: must be some string");
    }
    let base = &s[..index];
    let null_s = null_substring(base);
    if null_s > 0 && null_s == index {
        return BigDecimal::zero()
    }
    let rep = u64::from_str(&s[index + 1..])
        .expect("Incorrect syntax after *: must be u64");
    if rep == 0 {
        return BigDecimal::zero()
    }
    BigDecimal::from(
        base.repeat(rep as usize).as_str()
    )
}
