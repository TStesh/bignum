#![allow(unused)]
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign};
use bignum::Big;

//-------------------------------------------------------------------------------------------------
// Матрицы больших чисел 2х2
#[derive(Clone)]
struct Mat(Box<Vec<Big>>);

impl Mat {
    // Нулевая матрица
    fn zero() -> Self {
        Self(Box::new((0..4).into_iter().map(|_| Big::zero()).collect()))
    }
    // единичная матрица
    fn one() -> Self {
        Self(Box::new(vec![Big::one(), Big::zero(), Big::zero(), Big::one()]))
    }
    // конструктор произвольной матрицы
    fn some(a: Big, b: Big, c: Big, d: Big) -> Self {
        Self(Box::new(vec![a, b, c, d]))
    }
    // квадрат матрицы
    fn sqr(&self) -> Self {
        // (a, b) (a, b) = (a^2+bc,  b*(a+d)
        // (c, d) (c, d)   (c*(a+d), bc+d^2)
        let x = (*self).0[0].sqr(); // a^2
        let y = (*self).0[3].sqr(); // d^2
        let z = (*self).0[0].clone() + (*self).0[3].clone(); // a + d
        let v = (*self).0[1].clone() * (*self).0[2].clone(); // bc
        Self::some(x + v.clone(), (*self).0[1].clone() * z.clone(),
                   (*self).0[2].clone() * z, y + v)
    }
    // степень матрицы
    fn exp(&mut self, p: u32) { self.0 = (pow(self, p)).0 }
}

impl Add for Mat {
    type Output = Mat;
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = Box::new(Vec::new());
        for i in 0..4 {
            v.push(self.0[i].clone() + rhs.0[i].clone())
        };
        Self(v)
    }
}

impl AddAssign for Mat {
    fn add_assign(&mut self, rhs: Self) { self.0 = ((*self).clone() + rhs).0; }
}

impl Mul for Mat {
    type Output = Mat;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut v = Box::new(Vec::new());
        for i in (0..4).step_by(2) {
            for j in 1..3 {
                v.push(self.0[i].clone() * rhs.0[j - 1].clone() +
                    self.0[i + 1].clone() * rhs.0[j + 1].clone())
            }
        }
        Self(v)
    }
}

impl MulAssign for Mat {
    fn mul_assign(&mut self, rhs: Self) { self.0 = ((*self).clone() * rhs).0; }
}

// Сравнение двух матриц
impl PartialEq for Mat {
    fn eq(&self, other: &Self) -> bool {
        let x = true;
        for i in 0..4 { if self.0[i] == other.0[i] { return x } }
        x
    }
}

impl Display for Mat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\n{} {}", self.0[0].to_str(), self.0[1].to_str(),
               self.0[2].to_str(), self.0[3].to_str())
    }
}

// Вычисление степени матрицы по определению
fn pow(mat: &Mat, m: u32) -> Mat {
    if m == 0 { return Mat::one() }
    if m == 1 { return (*mat).clone() }
    if m == 2 { return mat.sqr() }
    let x = pow(mat, m >> 1).sqr();
    if m & 1 == 0 { return x }
    (*mat).clone() * x
}

// Вычисление степени матрицы бинарным методом
fn bin_pow(mat: &Mat, m: u32) -> Mat {
    if m == 0 { return Mat::one() }
    if m == 1 { return (*mat).clone() }
    if m == 2 { return mat.sqr() }
    let mut p = m >> 1;
    let mut p_prev = m;
    let mut y = Mat::one();
    let mut z = (*mat).clone();
    while p > 0 {
        if p_prev & 1 > 0 { y *= z.clone() }
        z = z.sqr();
        p_prev = p;
        p >>= 1;
    }
    y * z
}

// Вычисление n-го числа Фибоначии
pub fn fib(n: u32) -> Big {
    let mut x = Mat::some(Big::one(), Big::one(), Big::one(), Big::zero());
    x.exp(n - 1);
    x.0[0].clone()
}
