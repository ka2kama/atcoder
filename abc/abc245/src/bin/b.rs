#![allow(nonstandard_style)]

use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! { N: usize, A: [i64; N] };
    let a: HashSet<i64> = A.into_iter().collect();
    let b: HashSet<i64> = (0..=a.iter().max().unwrap() + 1).collect();
    let ans = (&b - &a).into_iter().min().unwrap();
    println!("{}", ans);
}
