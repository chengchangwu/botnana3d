extern crate time;
extern crate nalgebra as na;
use time::precise_time_ns;
use std::ops::Mul;
use na::{Vector3};

fn main() {
    let ones = Vector3::new(1.0, 1.0, 1.0);
    let zeros = Vector3::new(1.0, 1.0, 1.0);
    let mut v1 = zeros.clone();
    let v2 = ones.clone();
    let start = precise_time_ns();
    for _i in 0..1000 {
        let s = v1.mul(v2);
        v1 = v1.mul(s);
    }
    let elapsed = precise_time_ns() - start;
    println!("elapsed: {} ns", elapsed);
}