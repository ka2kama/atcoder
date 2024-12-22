#![allow(unused, nonstandard_style)]

use std::{iter, str};

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
struct Foo ;

#[fastout]
fn main() {
    input! { mut A: [i64; 3], }
    A.sort();
    let ans = if A[1] - A[0] == A[2] - A[1] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
