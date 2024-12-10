#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_integer::Integer;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { A: usize, B: usize, }
    let ans = A.lcm(&B);
    println!("{}", ans);
}
