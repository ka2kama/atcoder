#![allow(unused, nonstandard_style)]

use std::collections::HashSet;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

const A_MAX: usize = 100_010;
const B_MAX: u32 = 50;

#[fastout]
fn main() {
    input! { N: usize, }
    let s = (2..=A_MAX)
        .flat_map(|a| (2..=B_MAX).map(move |b| a.pow(b)).take_while(|&p| p <= N))
        .unique()
        .count();

    let ans = N - s;
    println!("{}", ans);
}
