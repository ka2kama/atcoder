#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, AsciiString};
use itertools::Itertools;
use maplit::hashmap;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};
use std::collections::HashSet;

const TEN_THOUSAND_DIGITS: [[usize; 4]; 10000] = {
    let mut i = 0;
    let mut v: [[usize; 4]; 10000] = [[0, 0, 0, 0]; 10000];
    while i < 10000 {
        v[i] = to_digit_array(i);
        i += 1;
    }
    v
};

const fn to_digit_array(mut num: usize) -> [usize; 4] {
    let mut digits = [0, 0, 0, 0];

    let mut i = 3;
    loop {
        digits[i] = num % 10;
        if i == 0 {
            break digits;
        }
        num /= 10;
        i -= 1;
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    S: Vec<AsciiChar>,
}

/// to avoid code completion issues caused by input macro
fn read_input() -> Input {
    input! {
        S: AsciiString,
    }

    Input { S: S.into() }
}

#[fastout]
fn main() {
    let Input { S } = read_input();

    let mandatory = S
        .iter()
        .enumerate()
        .filter_map(|(i, &ch)| if ch == AsciiChar::o { Some(i) } else { None })
        .collect_vec();

    let ans = TEN_THOUSAND_DIGITS
        .into_iter()
        .filter(|digits| {
            digits.iter().all(|&d| S[d] != AsciiChar::x)
                && mandatory.iter().all(|d| digits.contains(d))
        })
        .count();

    println!("{}", ans);
}
