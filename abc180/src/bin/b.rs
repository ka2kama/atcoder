#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [i64; N], }
    let v: Vec<i64> = A.into_iter().map(|x| x.abs()).collect();
    let ans_m: i64 = v.iter().sum();
    let ans_e: f64 = {
        let s: i64 = v.iter().map(|x| x.pow(2)).sum();
        s.to_f64().unwrap().sqrt()
    };
    let ans_c: i64 = *v.iter().max().unwrap();
    println!("{}", ans_m);
    println!("{:.15}", ans_e);
    println!("{}", ans_c);
}
