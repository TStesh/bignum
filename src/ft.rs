use std::collections::HashMap;
use std::f64::consts::PI;
use std::iter::Rev;
use crate::complex::Complex;
use rand::Rng;
use rayon::prelude::*;

pub struct RevCash {
    data: HashMap<usize, Vec<usize>>
}

impl RevCash {
    pub fn new(limit: usize) -> Self {
        let mut data = HashMap::new();
        let mut n = 2usize;
        for _ in 0..limit {
            data.insert(n, rev_swap(n));
            n <<= 1;
        }
        Self { data }
    }
    pub fn get(&self, n: usize) -> &[usize] {
        // println!("cashe get: {n}");
        self.data.get(&n).unwrap().as_slice()
    }
}

// Дискретное преобразование Фурье по определению
// функция нужна как эталон для проверки разных быстрых реализаций
// На входе a - срез вектора комплексных чисел
// rev - прямое (true) или обратное (false) преобразование
pub fn dft(input_arr: &[Complex], rev: bool) -> Vec<Complex> {
    let l = input_arr.len();
    let mut res = Vec::new();
    let angle = 2. * PI / l as f64 * if rev { -1. } else { 1. };
    let mut i_angle = 0.;
    for i in 0..l {
        let mut s = Complex::zero();
        let mut j_angle: f64 = i_angle;
        for j in 0..l {
            s = s + input_arr[j] * Complex(j_angle.cos(), j_angle.sin());
            j_angle += i_angle;
        }
        if rev { res.push(s / l as f64) } else { res.push(s) }
        i_angle += angle;
    }
    res
}

// Быстрое преобразование Фурье (build-in)
// rev = false прямое и rev = true обратное
// Требование: размер входного массива является степенью двойки
// Не рекурсивная реализация, доп. память не требуется!
pub fn fft(input_arr: &mut [Complex], rev_indexes: &[usize], rev: bool) {
    let n = input_arr.len();
    if n == 1 { return }
    // расстановка элементов вектора для нижнего уровня рекурсии
    for i in 1..n {
        let j = rev_indexes[i];
        if i < j { input_arr.swap(i, j); }
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
                let u = input_arr[i + j];
                let v = w[j] * input_arr[i + j + m];
                input_arr[i + j] = u + v;
                input_arr[i + j + m] = u - v;
            }
        }
        len <<= 1;
    }
    if rev {
        for i in 0..n {
            input_arr[i] /= n as f64;
        }
    }
}

// поразрядно обратная перестановка
// можно вычислить и закэшировать все перестановки
pub fn rev_swap(n: usize) -> Vec<usize> {
    // println!("rev_swap: {n}");
    let mut v = Vec::with_capacity(n);
    unsafe { v.set_len(n) }
    v.fill(0);
    let mut j = 0;
    for ind in 1..n {
        let mut bit = n >> 1;
        while j >= bit {
            j -= bit;
            bit >>= 1;
        }
        j += bit;
        v[ind] = j;
    }
    v
}

// Генератор массива случайных комплексных чисел
// Будем делать это параллельно
// (min_v, max_v) - интервал значений re и im
// size - размер вектора
pub fn random_complex_vec(min_v: f64, max_v: f64, size: usize) -> Vec<Complex> {
    (0..size).into_par_iter()
        .map(|i|
            Complex(
                rand::thread_rng().gen_range(min_v..max_v),
                rand::thread_rng().gen_range(min_v..max_v)
            )
        )
        .collect()
}
