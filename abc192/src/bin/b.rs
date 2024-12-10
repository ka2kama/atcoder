#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { S: Chars, }
    let ans = S.iter().enumerate().all(|(i, c)| {
        if i % 2 == 0 {
            c.is_lowercase()
        } else {
            c.is_uppercase()
        }
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
