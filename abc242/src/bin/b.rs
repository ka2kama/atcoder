#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { mut S: Chars, };
    S.sort();
    let ans: String = S.into_iter().collect();
    println!("{}", ans);
}
