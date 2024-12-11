#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, AsciiString};
use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

struct Input {
    N: usize,
    A: Vec<i64>,
}

/// to avoid code completion issues caused by input macro
fn read_input() -> Input {
    input! {
        N: usize,
        A: [i64; N],
    }

    Input { N, A }
}

#[fastout]
fn main() {
    todo!();
    let Input { N, A } = read_input();
    
    
    let ans = "";
    println!("{}", ans);
}
