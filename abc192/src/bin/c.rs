#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, AsciiStr, AsciiString, IntoAsciiString, ToAsciiChar};
use itertools::{fold, Itertools};
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::source::{Readable, Source};
use proconio::{derive_readable, fastout, input};
use smallvec::SmallVec;
use std::cmp::Reverse;
use std::io::BufRead;

enum AsciiChars {}

impl Readable for AsciiChars {
    type Output = Vec<AsciiChar>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<AsciiChar> {
        let token = source.next_token_unwrap();
        token.into_ascii_string().unwrap().into()
    }
}

fn main() {
    input! {
        N: u64, K: usize
    }

    let ans = (0..K).fold(N, |acc, _| f(acc));
    println!("{}", ans);
}

fn f(mut n: u64) -> u64 {
    let v = {
        let mut v: SmallVec<[u64; 10]> = SmallVec::new();
        while n > 0 {
            v.push(n % 10);
            n /= 10;
        }
        v.sort_unstable();
        v
    };
    let asc = v.iter().rev().fold(0u64, |acc, &x| acc * 10 + x);
    let desc = v
        .iter()
        .skip_while(|&&x| x == 0)
        .fold(0u64, |acc, &x| acc * 10 + x);
    asc - desc
}
