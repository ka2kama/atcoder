#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, S: Chars, T: Chars, }
    let ans = (0..N).flat_map(|i| vec![S[i], T[i]]).join("");
    println!("{}", ans);
}
