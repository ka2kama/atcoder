#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { H: usize, W: usize, A: [[i64; W]; H] }
    let v: Vec<i64> = A.into_iter().flatten().collect();
    let min = *v.iter().min().unwrap();
    let ans: i64 = v.iter().map(|x| x - min).sum();
    println!("{}", ans);
}
