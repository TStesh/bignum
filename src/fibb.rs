use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign};
use crate::big::BigDecimal;
use crate::oper::{add_vec, mul_vec};

//-------------------------------------------------------------------------------------------------
// Матрицы больших чисел 2х2
struct Mat {
    a: BigDecimal,
    b: BigDecimal,
    c: BigDecimal,
    d: BigDecimal
}

impl Mat {
    // единичная матрица
    fn one() -> Self {
        Self {
            a: BigDecimal::one(), b: BigDecimal::zero(),
            c: BigDecimal::zero(), d: BigDecimal::one()
        }
    }
    // квадрат матрицы
    fn sqr(&self) -> Self {
        // (a, b) (a, b) = (a^2+bc,  b*(a+d)
        // (c, d) (c, d)   (c*(a+d), bc+d^2)
        let ad= add_vec(&self.a.digits,&self.d.digits);
        let bc= mul_vec(&self.b.digits,&self.c.digits);
        Self {
            a: BigDecimal::from(add_vec(&self.a.sqr().digits,&bc).as_slice()),
            b: BigDecimal::from(mul_vec(&self.b.digits,&ad).as_slice()),
            c: BigDecimal::from(mul_vec(&self.c.digits,&ad).as_slice()),
            d: BigDecimal::from(add_vec(&self.d.sqr().digits,&bc).as_slice())
        }
    }
    // степень матрицы
    fn exp(&mut self, p: u32) { *self = pow(&self, p) }
}

impl Add for Mat {
    type Output = Mat;
    fn add(self, rhs: Self) -> Self::Output {
        add_mat(&self, &rhs)
    }
}

impl AddAssign for Mat {
    fn add_assign(&mut self, rhs: Self) {
        *self = add_mat(&self, &rhs);
    }
}

impl Mul for Mat {
    type Output = Mat;
    fn mul(self, rhs: Self) -> Self::Output {
        mul_mat(&self, &rhs)
    }
}

impl MulAssign for Mat {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mul_mat(&self, &rhs);
    }
}

impl Display for Mat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\n{} {}", self.a, self.b, self.c, self.d)
    }
}

// Вычисление степени матрицы по определению
fn pow(mat: &Mat, m: u32) -> Mat {
    if m == 0 { return Mat::one() }
    let x = pow(mat, m >> 1).sqr();
    if m & 1 == 0 { x } else { mul_mat(mat, &x) }
}

// Вычисление n-го числа Фибоначии
pub fn fib(n: u32) -> BigDecimal {
    let mut f = Mat {
        a: BigDecimal::one(), b: BigDecimal::one(),
        c: BigDecimal::one(), d: BigDecimal::zero()
    };
    f.exp(n - 1);
    f.a
}

// p + q
fn add_mat(p: &Mat, q: &Mat) -> Mat {
    Mat {
        a: BigDecimal::from(add_vec(&p.a.digits,&q.a.digits).as_slice()),
        b: BigDecimal::from(add_vec(&p.b.digits,&q.b.digits).as_slice()),
        c: BigDecimal::from(add_vec(&p.c.digits,&q.c.digits).as_slice()),
        d: BigDecimal::from(add_vec(&p.d.digits,&q.d.digits).as_slice())
    }
}

// p * q
fn mul_mat(p: &Mat, q: &Mat) -> Mat {
    // (a, b) (a, b) = (aa+bc, ab+bd)
    // (c, d) (c, d)   (ca+dc, cb+dd)
    Mat {
        a: BigDecimal::from(add_vec(
            &mul_vec(&p.a.digits,&q.a.digits),
            &mul_vec(&p.b.digits,&q.c.digits)
        ).as_slice()),
        b: BigDecimal::from(add_vec(
            &mul_vec(&p.a.digits,&q.b.digits),
            &mul_vec(&p.b.digits,&q.d.digits)
        ).as_slice()),
        c: BigDecimal::from(add_vec(
            &mul_vec(&p.c.digits,&q.a.digits),
            &mul_vec(&p.d.digits,&q.c.digits)
        ).as_slice()),
        d: BigDecimal::from(add_vec(
            &mul_vec(&p.c.digits,&q.b.digits),
            &mul_vec(&p.d.digits,&q.d.digits)
        ).as_slice())
    }
}