//use std::cmp::min;
//use bignum::{benchmark, Big};
//use rand::Rng;
// mod rational;
// use crate::rational::Ratio;
mod big;
mod ft;
mod oper;
mod complex;
mod fibb;

use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::big::BigDecimal;
use crate::fibb::fib;
use crate::ft::{rev_swap, RevCash};

lazy_static!(
    pub static ref REV_CASH: RevCash = RevCash::new(22);
);

fn main() {

    let start = std::time::Instant::now();
    let x = fib(1_000_000);
    println!("Duration: {:?}", start.elapsed());
    // println!("{x}");

    // let a = BigDecimal::from("9856*100");
    // let b = BigDecimal::from("2314*200");

    /*
    let mut r = Ratio::from(1);
    for x in 2..=20 {
        r += Ratio::reciproc(x);
    }
    println!("{r}");

    let start = std::time::Instant::now();
    let y = fibb::fib(1_000_000);
    println!("Duration: {:?}", start.elapsed());
    println!("{}", y.to_str().len());
    //println!("Duration: {:?}", benchmark(1_000_000, 10));
    */
}
