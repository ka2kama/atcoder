#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::input;
use proconio::source::Readable;
fn main() {
    input! {
        A: isize, B: isize, C: isize,
    }
    let ans = match [A, B, C].into_iter().unique().count() {
        3 => 0,
        _ => A ^ B ^ C,
    };
    println!("{}", ans);
}
