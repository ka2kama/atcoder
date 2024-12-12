#![allow(unused, nonstandard_style)]

use bstr::B;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: i64, A: i64, B: i64 }
    let ans = N - A + B;
    println!("{}", ans);
}
