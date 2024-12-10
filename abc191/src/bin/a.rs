#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { V: usize, T: usize, S: usize, D: usize }
    let ans = if V * T <= D && D <= V * S {
        "No"
    } else {
        "Yes"
    };
    println!("{}", ans);
}
