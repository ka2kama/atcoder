#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize, S: usize, D: usize,
        A: [(usize, usize); N]
    };
    let ans = A.into_iter().any(|(eisyo, iryoku)| eisyo < S && iryoku > D);
    println!("{}", if ans { "Yes" } else { "No" });
}
