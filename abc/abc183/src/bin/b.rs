#![allow(unused, nonstandard_style)]

use std::convert::TryFrom;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[fastout]
fn main() {
    input! { S: Point, G:Point, };
    let ans = S.x - S.y * (G.x - S.x) / (-G.y - S.y);
    println!("{}", ans);
}
