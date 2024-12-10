#![allow(unused, nonstandard_style)]

use std::cmp::max;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N],
        B: [i64; N],
    };
    let a_max = A.into_iter().max().unwrap();
    let b_min = B.into_iter().min().unwrap();
    let ans = max(b_min - a_max, -1) + 1;
    println!("{}", ans);
}
