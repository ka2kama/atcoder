#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: i64, A: i64, B: i64 }
    let ans = N - A + B;
    println!("{}", ans);
}
