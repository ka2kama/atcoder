#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, AsciiString, IntoAsciiString};
use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};
use std::{iter, mem};

#[fastout]
fn main() {
    input! {
        N: usize,
        mut S: AsciiString,
        Q: usize,
        QS: [(u8, usize, usize); Q]
    }

    let mut xs: Vec<AsciiChar> = S.into();
    let mut ys = xs.split_off(N);
    for (T, A, B) in QS {
        match T {
            1 if A != B => {
                // ensure i < j
                let (i, j) = if A < B {
                    (A - 1, B - 1)
                } else {
                    (B - 1, A - 1)
                };
                match (i < N, j < N) {
                    (true, true) => xs.swap(i, j),
                    (false, false) => ys.swap(i - N, j - N),
                    _ => mem::swap(&mut xs[i], &mut ys[j - N]),
                }
            }
            2 => mem::swap(&mut xs, &mut ys),
            _ => {}
        }
    }

    print!("{}", xs.into_ascii_string().unwrap());
    print!("{}", ys.into_ascii_string().unwrap());
    println!();
}
