#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [(i64, i64); N], }
    let ans: i64 = A.iter().flat_map(|&(s, e)| s..=e).sum();
    println!("{}", ans);
}
