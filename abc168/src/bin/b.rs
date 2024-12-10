#![allow(unused, nonstandard_style)]

use std::fmt::format;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { K: usize, S: String, }
    let ans = if S.len() <= K {
        S
    } else {
        format!("{}...", &S[..K])
    };
    println!("{}", ans);
}
