#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, }
    let v = (1..=N.sqrt())
        .filter_map(|i| {
            if N % i == 0 {
                let mut p = vec![i, N / i];
                p.dedup();
                Some(p)
            } else {
                None
            }
        })
        .flatten()
        .sorted();

    for ans in v {
        println!("{}", ans);
    }
}
