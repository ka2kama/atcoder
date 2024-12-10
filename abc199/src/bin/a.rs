#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { A: i64, B: i64, C: i64}
    let ans = A.pow(2) + B.pow(2) < C.pow(2);
    println!("{}", if ans { "Yes" } else { "No" });
}
