#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
#[derive(Debug)]
struct Sake {
    ml: i64,
    percent: i64,
}

#[fastout]
fn main() {
    input! { N: usize, X:i64, V: [Sake; N], }
    let lim = X * 100;
    let total_alcohols = V.iter().scan(0_i64, |total_alcohol, sake| {
        *total_alcohol += sake.ml * sake.percent;
        Some(*total_alcohol)
    });
    let ans = total_alcohols
        .enumerate()
        .find(|(_, total_alcohol)| *total_alcohol > lim)
        .map_or(-1, |(i, _)| (i + 1) as i64);
    println!("{}", ans);
}
