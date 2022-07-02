mod fibb;

use std::cmp::min;
use bignum::{benchmark, Big};
use rand::Rng;

fn main() {
    let start = std::time::Instant::now();
    let y = fibb::fib(1_000_000);
    println!("Duration: {:?}", start.elapsed());
    println!("{}", y.to_str().len());
    //println!("Duration: {:?}", benchmark(1_000_000, 10));
}
