#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, S: usize, D: usize,
        SPELLS: [(usize, usize); N],
    }
    let ans = SPELLS.iter().any(|&(x, y)| x < S && y > D);
    println!("{}", if ans { "Yes" } else { "No" });
}
