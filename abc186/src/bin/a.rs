#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, W: usize, }
    let ans = N / W;
    println!("{}", ans);
}
