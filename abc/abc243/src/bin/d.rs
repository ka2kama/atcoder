#![allow(nonstandard_style)]

use num_traits::ToPrimitive;
use proconio::derive_readable;
use proconio::{fastout, input};

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[fastout]
fn main() {
    input! { A: i64, B: i64 };
    let ans = "";
    println!("{}", ans);
}
