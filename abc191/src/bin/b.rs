#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, X: i64,
        A: [i64; N]
    }

    let mut ys = A.into_iter().filter(|&y| y != X);
    if let Some(y) = ys.next() {
        print!("{}", y);
        for y in ys {
            print!(" {}", y);
        }
        println!();
    }
}
