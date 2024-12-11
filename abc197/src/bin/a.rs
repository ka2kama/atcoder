#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, AsciiString, IntoAsciiString};
use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

struct Input {
    S: Vec<AsciiChar>,
}

/// to avoid code completion issues caused by input macro
fn read_input() -> Input {
    input! {
        S: AsciiString
    }

    Input { S: S.into() }
}

#[fastout]
fn main() {
    let Input { S } = read_input();

    println!("{}{}{}", S[1], S[2], S[0]);
}
