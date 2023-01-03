#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { N: usize, A: [i64; N] };
    let ans = A.iter().unique().count();
    println!("{:?}", ans);
}
