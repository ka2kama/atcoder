#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { A: i32, B: i32, C: i32 }
    let takahashi_win = match C {
        0 => A > B,
        _ => A >= B,
    };
    println!("{}", if takahashi_win { "Takahashi" } else { "Aoki" });
}
