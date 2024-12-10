#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { A: f64, B: f64, C: f64, X: f64 };
    let ans = if X <= A {
        1.
    } else if X > B {
        0.0
    } else {
        C / (B - A)
    };
    println!("{:.12}", ans);
}
