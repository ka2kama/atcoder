#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
#[derive(Debug)]
struct Shop {
    time: i64,
    price: i64,
    stock: i64,
}

#[fastout]
fn main() {
    input! { N: usize, A: [Shop; N], }
    let ans = A
        .iter()
        .filter_map(|s| {
            if s.time < s.stock {
                Some(s.price)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(-1);
    println!("{}", ans);
}
