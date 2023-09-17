use rayon::prelude::*;
use crate::ft::{fft, rev_swap};
use crate::complex::Complex;
use crate::REV_CASH;

// a + b
pub fn add_vec(arr_a: &[u8], arr_b: &[u8]) -> Vec<u8> {
    // проверка сложения с 0
    let v = [0u8].as_slice();
    if arr_a == v { return arr_b.to_vec() }
    if arr_b == v { return arr_a.to_vec() }
    // во все тяжкие
    if arr_a.len() >= arr_b.len() {
        add_vec_loc(arr_a, arr_b)
    } else {
        add_vec_loc(arr_b, arr_a)
    }
}

// a + b
// len(a) >= len(b)
fn add_vec_loc(a: &[u8], b: &[u8]) -> Vec<u8> {
    let size_a = a.len();
    let size_b = b.len();
    let mut res = Vec::with_capacity(size_a + 1);
    unsafe { res.set_len(size_a + 1) }
    let mut carry = 0;
    for i in 0..size_a {
        carry += a[i] + if i < size_b { b[i] } else { 0 };
        if carry > 9 {
            res[i] = carry - 10;
            carry = 1;
        } else {
            res[i] = carry;
            carry = 0;
        }
    }
    if carry > 0 {
        res[size_a] = carry;
    } else {
        res.truncate(size_a);
    };
    res
}

// a * b
pub fn mul_vec(arr_a: &[u8], arr_b: &[u8]) -> Vec<u8> {
    // проверка умножения на 0
    let v0 = vec![0u8];
    let vs0 = v0.as_slice();
    if arr_a == vs0 || arr_a == vs0 { return v0 }
    // проверка умножения на 1
    let v = [1u8].as_slice();
    if arr_a == v { return arr_b.to_vec() }
    if arr_b == v { return arr_a.to_vec() }
    // во все тяжкие
    if arr_a.len() >= arr_b.len() {
        mul_vec_loc(arr_a, arr_b)
    } else {
        mul_vec_loc(arr_b, arr_a)
    }
}

// a * b
// len(a) >= len(b)
fn mul_vec_loc(a: &[u8], b: &[u8]) -> Vec<u8> {
    // находим ближайшую к размеру большего вектора степень 2
    let size = a.len();
    let mut n = size.checked_next_power_of_two().unwrap() << 1;
    // используем кэш для обратной расстановки индексов
    let idx = REV_CASH.get(n);
    // Обратное БПФ над нашими двумя массивами (потенциально параллельно)
    let (x, y) = rayon::join(
        || go_fft(&a, idx, n),
        || go_fft(&b, idx, n)
    );
    // Сворачиваем x и y
    let mut red_xy: Vec<Complex> = (0..n).into_par_iter()
        .map(|i| x[i] * y[i])
        .collect();
    // Прямое БПФ над сверткой
    fft(&mut red_xy, idx, true);
    // Получаем результат
    normalize(&red_xy, n, size)
}

// Prepare + FFT
fn go_fft(inp_arr: &[u8], rev_indexes: &[usize], size: usize) -> Vec<Complex> {
    let mut res = vec![Complex::zero(); size];
    for i in 0..inp_arr.len() { res[i].0 = inp_arr[i] as f64; }
    fft(&mut res, rev_indexes, false);
    res
}

// a * a
pub fn sqr(arr_a: &[u8]) -> Vec<u8> {
    if arr_a == [0u8].as_slice() { return vec![0u8] }
    if arr_a == [1u8].as_slice() { return arr_a.to_vec() }
    // Находим ближайшую к размеру большего вектора степень 2
    let size = arr_a.len();
    let mut n = size.checked_next_power_of_two().unwrap() << 1;
    let idx = REV_CASH.get(n);
    // Готовим расчетный массив комплексных чисел
    let mut x = vec![Complex::zero(); n];
    for i in 0..size { x[i].0 = arr_a[i] as f64; }
    // Выполняем обратное БПФ над расчетным массивом
    fft(&mut x, idx, false);
    // Сворачиваем x
    let mut red_x: Vec<Complex> = (0..n).into_par_iter()
        .map(|i| x[i] * x[i])
        .collect();
    // Выполняем прямое БПФ над сверткой
    fft(&mut red_x, idx, true);
    // Получаем результат
    normalize(&red_x, n, size)
}

// Normalize
// [Complex] -> [u8]
// размер выходного числа дне должен превышать 2 * (len(a) + 1)
fn normalize(inp_arr: &[Complex], size: usize, src_size: usize) -> Vec<u8> {
    let mut res_size = (src_size + 1) << 1;
    if res_size > size { res_size = size };
    let mut res = Vec::with_capacity(res_size);
    unsafe { res.set_len(res_size) }
    res.fill(0);
    let mut carry = 0u32;
    let mut x;
    for i in 0..res_size {
        x = (inp_arr[i].0 + 0.5) as u32 + carry;
        carry = x / 10;
        res[i] = (x % 10) as u8;
    }
    // убираем вспомогательные 0 в начале числа
    for i in (0..res_size).rev() {
        if res[i] != 0 {
            res.truncate(i + 1);
            break
        }
    }
    res
}
