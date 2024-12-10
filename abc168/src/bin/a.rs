#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: Chars, }
    let ans = match N.last().unwrap() {
        '2' | '4' | '5' | '7' | '9' => "hon",
        '0' | '1' | '6' | '8' => "pon",
        '3' => "bon",
        _ => unreachable!(),
    };
    println!("{}", ans);
}
