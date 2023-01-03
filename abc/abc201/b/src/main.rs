#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [(String, usize); N]
    };
    let (ans, _) = A
        .into_iter()
        .sorted_by_key(|&(_, height)| height)
        .rev()
        .nth(1)
        .unwrap();
    println!("{}", ans);
}
