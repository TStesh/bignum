#![allow(unused)]
mod complex;
mod rational;

use complex::Complex;
use std::cmp::max;
use std::f64::consts::PI;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;

//------------------------------------------------------------------------------------------------
// Большие числа
#[derive(Clone)]
pub struct Big(pub Vec<u8>);

impl Big {
    // int -> big
    pub fn from_num(number: u128) -> Self {
        if number < 10 { Self(vec![number as u8]) } else { Self(to_digits(number)) }
    }
    // string -> big
    pub fn from_str(str: String) -> Self {
        Self( str.chars().into_iter().rev()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect() )
    }
    // big -> string
    pub fn to_str(&self) -> String {
        (&self.0).into_iter().rev()
            .map(|x| x.to_string())
            .reduce(|a, b| a + b.as_str())
            .unwrap()
    }
    // big -> vector<complex> for FFT
    pub fn to_complex(&self) -> Vec<Complex> {
        let mut v = Vec::new();
        for x in &self.0 { v.push(Complex(*x as f64, 0.)) }
        let size = v.len();
        let mut n = 1;
        while n < size { n <<= 1; }
        n <<= 1;
        v.resize(n, Complex::zero());
        v
    }
    // square
    pub fn sqr(&self) -> Self { Self(sqr(&self.0)) }
    // zero
    pub fn zero() -> Self { Self(vec![0]) }
    // one
    pub fn one() -> Self { Self(vec![1]) }
}

// big + big -> big
impl Add for Big {
    type Output = Big;
    fn add(self, rhs: Self) -> Self::Output {
        Big(add_vec(&self.0, &rhs.0))
    }
}

// big += big
impl AddAssign for Big {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = add_vec(&self.0, &rhs.0);
    }
}

// big + u128 -> big
impl Add<u128> for Big {
    type Output = Big;
    fn add(self, rhs: u128) -> Self::Output {
        self + Big::from_num(rhs)
    }
}

// u128 + big -> big
impl Add<Big> for u128 {
    type Output = Big;
    fn add(self, rhs: Big) -> Self::Output {
        Big::from_num(self) + rhs
    }
}

// big * big
impl Mul<Big> for Big {
    type Output = Big;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(multiply(&self.0, &rhs.0))
    }
}

impl PartialEq for Big {
    fn eq(&self, other: &Self) -> bool {
        let x = true;
        let len = (self.0.len(), other.0.len());
        if len.0 != len.1 { return false }
        for i in 0..len.0 { if self.0[i] != other.0[i] { return false } }
        x
    }
}

//------------------------------------------------------------------------------------------------
// Получить цифры натурального числа в виде вектора
fn to_digits(mut number: u128) -> Vec<u8> {
    let mut v = Vec::new();
    while number > 0 {
        v.push((number % 10) as u8);
        number /= 10;
    }
    v
}

// вектор -> число
// [a0, a1, a2] -> a0 + 10*a1 + 100*a2
fn to_number(a: &Vec<u8>) -> u128 {
    let mut v = 0u128;
    let mut k = 1;
    for i in 0..a.len() { v += k * a[i] as u128; k *= 10; }
    v
}

// Сложить векторы
fn add_vec(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let len = (a.len(), b.len());
    // Проверка на сложение с 0
    if len.0 == 1 && a[0] == 0 { return b.clone() }
    if len.1 == 1 && b[0] == 0 { return a.clone() }
    // Проверка на маленькие числа
    // размер u128 - это гарантированно 37 десятичных разрядов (10^37<3*10^38<2^128-1<4*10^38)
    // складываем как обычные целые
    if len.0 + len.1 <= 37 {
        return to_digits(to_number(&a) + to_number(&b))
    }
    let size = max(len.0, len.1);
    let mut res = a.clone();
    if len.0 < len.1 { res.resize(len.1, 0); }
    let mut carry = 0;
    for i in 0..size {
        carry += res[i] + if i < len.1 { b[i] } else { 0 };
        res[i] = carry % 10;
        carry /= 10;
    }
    if carry > 0 { res.push(carry) }
    res
}

//------------------------------------------------------------------------------------------------
// Вывод вектора на экран
fn print<T: Display>(name: &str, v: &Vec<T>) {
    print!("{}: ", name);
    let n = v.len();
    for i in 0..(n - 1) { print!("{}, ", v[i]); }
    println!("{}", v[n - 1]);
}

// vector<Complex> -> Vec<u32>
fn complex_vec_to_vec_u32(v: &Vec<Complex>) -> Vec<u32> {
    let mut w = Vec::new();
    for x in v {
        w.push(((*x).0 + 0.5) as u32)
    }
    w
}

//------------------------------------------------------------------------------------------------
// Дискретное преобразование Фурье по определению
// функция нужна как эталон для проверки разных быстрых реализаций
fn dft(a: &Vec<Complex>, rev: bool) -> Vec<Complex> {
    let l = a.len();
    let mut res = Vec::new();
    let angle = 2. * PI / l as f64 * if rev { -1. } else { 1. };
    for i in 0..l {
        let mut s = Complex::zero();
        let i_angle = angle * i as f64;
        for j in 0..l {
            let j_angle = i_angle * j as f64;
            let x = Complex(j_angle.cos(), j_angle.sin());
            let y = a[j];
            s = s + y * x;
        }
        if rev { res.push(s / l as f64) } else { res.push(s) }
    }
    res
}

// поразрядно обратная перестановка
fn rev(n: usize) -> Vec<usize> {
    let mut v = vec![0];
    let mut j = 0;
    for _ in 1..n {
        let mut bit = n >> 1;
        while j >= bit { j -= bit; bit >>= 1; }
        j += bit;
        v.push(j);
    }
    v
}

// Быстрое преобразование Фурье
// rev = false прямое и rev = true обратное
// Требование: размер входного массива является степенью двойки
// Не рекурсивная реализация, доп. память не требуется!
fn fft(a: &mut Vec<Complex>, rev_indexes: &Vec<usize>, rev: bool) {
    let n = a.len();
    if n == 1 { return }
    // расстановка элементов вектора для нижнего уровня рекурсии
    for i in 1..n {
        let j = rev_indexes[i];
        if i < j { a.swap(i, j); }
    }
    let mut len = 2;
    while len <= n {
        let m = len >> 1;
        let angle = 2. * PI / len as f64 * if rev { -1.} else { 1. };
        let main_root = Complex(angle.cos(), angle.sin());
        let mut w = vec![Complex::one()];
        // вычисление степеней omega
        for i in 1..m { w.push(w[i - 1] * main_root); }
        // расчет
        for i in (0..n).step_by(len) {
            for j in 0..m {
                let u = a[i + j];
                let v = w[j] * a[i + j + m];
                a[i + j] = u + v;
                a[i + j + m] = u - v;
            }
        }
        len <<= 1;
    }
    if rev {
        for i in 0..n { a[i] /= n as f64; }
    }
}

// Умножение двух целых чисел
fn multiply(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let len = (a.len(), b.len());
    // Проверка умножения на 0
    if len.0 == 1 && a[0] == 0 { return a.clone() }
    if len.1 == 1 && b[0] == 0 { return b.clone() }
    // Проверка умножения на 1
    if len.0 == 1 && a[0] == 1 { return b.clone() }
    if len.1 == 1 && b[0] == 1 { return a.clone() }
    // Проверка на умножение маленьких чисел
    // размер u128 - это гарантированно 37 десятичных разрядов (10^37<3*10^38<2^128-1<4*10^38)
    if len.0 + len.1 <= 37 {
        return to_digits(to_number(&a) * to_number(&b))
    }
    // Применяем "тяжелую артиллерию" FFT
    // Догоняем размер векторов до степени двойки
    let mut n: usize = 1;
    let max_size = max(len.0, len.1);
    while n < max_size { n <<= 1; }
    n <<= 1;
    // Считаем обратно поразрядную перестановку индексов
    let rev_indexes = &rev(n);
    // Считаем FFT в 2 потока
    let (tx, rx) = mpsc::channel();
    for x in 0..2 {
        let s = tx.clone();
        let rev_ind = rev_indexes.clone();
        let v = vec![a.clone(), b.clone()];
        let size = vec![len.0, len.1];
        thread::spawn(move || {
            let mut f = vec![Complex::zero(); n];
            for i in 0..size[x] { f[i] = Complex(v[x][i] as f64, 0.); }
            fft(&mut f, &rev_ind, false);
            s.send(f).unwrap();
        });
    }
    // Свертка
    let res1 = rx.recv().unwrap();
    let res2 = rx.recv().unwrap();
    let mut f = (0..n).into_iter()
        .map(|i| res1[i] * res2[i])
        .collect();
    // Выполняем обратное преобразование Фурье
    fft(&mut f, &rev_indexes,true);
    // Формируем результат из вещественных частей элементов получившегося вектора
    let mut res = Vec::new();
    let mut carry = 0u32;
    // при записи результата выполняем нормализацию
    for i in 0..n {
        let mut x = (f[i].0 + 0.5) as u32;
        x += carry;
        carry = x / 10;
        x %= 10;
        res.push(x as u8);
    }
    // Убираем лишние нули
    let mut null_counter = 0;
    for i in 0..n { if res[n - i - 1] == 0 { null_counter += 1 } else { break } }
    if null_counter > 0 { res.truncate(n - null_counter) }
    res
}

// Квадрат числа
fn sqr(a: &Vec<u8>) -> Vec<u8> {
    let size = a.len();
    // Проверка умножения на 0
    if size == 1 && a[0] == 0 { return a.clone() }
    // Проверка умножения на 1
    if size == 1 && a[0] == 1 { return a.clone() }
    // Проверка на умножение маленьких чисел
    // размер u128 - это гарантированно 37 десятичных разрядов (10^37<3*10^38<2^128-1<4*10^38)
    if size < 19 {
        let x = to_number(&a);
        return to_digits(x * x)
    }
    // Применяем "тяжелую артиллерию" FFT
    // Догоняем размер вектора до степени двойки
    let mut n: usize = 1;
    while n < size { n <<= 1; }
    n <<= 1;
    // Считаем обратно поразрядную перестановку индексов
    let rev_indexes = &rev(n);
    // Считаем FFT
    let mut f = vec![Complex::zero(); n];
    for i in 0..size { f[i] = Complex(a[i] as f64, 0.); }
    fft(&mut f, &rev_indexes, false);
    // Свертка
    for i in 0..n {
        let x = f[i];
        f[i] *= x;
    }
    // Выполняем обратное преобразование Фурье
    fft(&mut f, &rev_indexes,true);
    // Формируем результат из вещественных частей элементов получившегося вектора
    let mut res = Vec::new();
    let mut carry = 0u32;
    // при записи результата выполняем нормализацию
    for i in 0..n {
        let mut x = (f[i].0 + 0.5) as u32;
        x += carry;
        carry = x / 10;
        x %= 10;
        res.push(x as u8);
    }
    // Убираем лишние нули
    let mut null_counter = 0;
    for i in 0..n { if res[n - i - 1] == 0 { null_counter += 1 } else { break } }
    if null_counter > 0 { res.truncate(n - null_counter) }
    res
}

// бенчмарк
// на входе - размер чисел и кол-во тестов
// возвращает минимальное значение времени умножения
pub fn benchmark(num_size: usize, qa_num: u32) -> Duration {
    let f = |x| if x { rand::thread_rng().gen_range(0u8..9u8) }
        else { rand::thread_rng().gen_range(1u8..9u8) };
    let mut times = Vec::with_capacity(qa_num as usize);
    for i in 0..qa_num {
        let mut a: Vec<u8> = vec![0; num_size];
        let mut b: Vec<u8> = vec![0; num_size];
        let mut x: bool;
        for j in 0..num_size {
            x = j == num_size - 1;
            a[j] = f(x);
            b[j] = f(x);
        }
        // считаем произведение нашим методом
        let start = std::time::Instant::now();
        multiply(&a, &b);
        times.push(start.elapsed());
    }
    times.into_iter().min().unwrap()
}
