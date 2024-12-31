#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_integer::Integer;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [i64; N], }
    let max = *A.iter().max().unwrap();
    let ans = (2..=max)
        .map(|i| (i, A.iter().filter(|&x| x % i == 0).count()))
        .max_by_key(|(_, cnt)| *cnt)
        .unwrap()
        .0;
    println!("{}", ans);
}
