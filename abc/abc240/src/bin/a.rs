#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { A: i64, B: i64 };
    let ans = if (A - B).abs() == 1 || (A == 1 && B == 10) || (A == 10 && B == 1) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
