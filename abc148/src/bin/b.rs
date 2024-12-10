#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, S: Chars, T: Chars, }
    let ans: String = S.iter().interleave(&T).collect();
    println!("{}", ans);
}
