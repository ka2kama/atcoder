#![allow(unused, nonstandard_style)]

use std::convert::TryFrom;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: i64, };
    let ans = N.max(0);
    println!("{}", ans);
}
