#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use maplit::hashset;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { a: i64, b: i64, c: i64, }
    let ans = match hashset![a, b, c].len() {
        3 => 0,
        _ => a ^ b ^ c,
    };
    println!("{}", ans);
}
