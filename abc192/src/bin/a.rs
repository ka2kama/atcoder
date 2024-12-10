#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { X: usize, }
    let ans = 100 - X % 100;
    println!("{}", ans);
}
