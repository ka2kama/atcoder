#![allow(unused, nonstandard_style)]

use std::cmp::Reverse;
use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: i64, K: usize, }
    let ans = itertools::iterate(N, |&x| f(x)).nth(K).unwrap();
    println!("{}", ans);
}

fn g1(x: i64) -> i64 {
    let s: String = x
        .to_string()
        .chars()
        .sorted_by_key(|&c| Reverse(c))
        .collect();
    s.parse().unwrap()
}

fn g2(x: i64) -> i64 {
    let s: String = x.to_string().chars().sorted().collect();
    s.parse().unwrap()
}

fn f(x: i64) -> i64 {
    g1(x) - g2(x)
}
