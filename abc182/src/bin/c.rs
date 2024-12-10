#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: i64, }
    let s = N.to_string();
    let v = s.chars().map(|c| c.to_digit(10).unwrap() % 3).collect_vec();
    let ans = match N % 3 {
        0 => 0,
        1 => {
            if v.iter().any(|&r| r == 1) {
                if s.len() <= 1 {
                    -1
                } else {
                    1
                }
            } else if s.len() <= 2 {
                -1
            } else {
                2
            }
        }
        2 => {
            if v.iter().any(|&r| r == 2) {
                if s.len() <= 1 {
                    -1
                } else {
                    1
                }
            } else if s.len() <= 2 {
                -1
            } else {
                2
            }
        }
        _ => unreachable!(),
    };
    println!("{}", ans);
}
