#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, }
    let ans = if N % 2 == 0 { "White" } else { "Black" };
    println!("{}", ans);
}
