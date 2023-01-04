#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: i64, K: i64, }
    let ans: i64 = (1..=N)
        .map(|i| i * 100)
        .map(|i| (1..=K).map(move |j| i + j))
        .flatten()
        .sum();
    println!("{}", ans);
}
