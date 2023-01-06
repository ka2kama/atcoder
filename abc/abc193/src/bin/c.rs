#![allow(unused, nonstandard_style)]

use std::collections::HashSet;
use std::iter;

use itertools::Itertools;
use maplit::hashset;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, }
    let set: HashSet<usize> = (2..=10_usize.pow(5) + 10)
        .flat_map(|a| (2..=50).map(move |b| a.pow(b)).take_while(|&p| p <= N))
        .collect();

    let ans = N - set.len();
    println!("{}", ans);
}
