#![allow(unused, nonstandard_style)]

use std::collections::HashSet;
use std::iter;
use std::iter::FromIterator;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { C: String, }
    let s: HashSet<char> = HashSet::from_iter(C.chars());
    let ans = if s.len() == 1 { "Won" } else { "Lost" };
    println!("{}", ans);
}
