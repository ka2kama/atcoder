#![allow(unused, nonstandard_style)]

use proconio::input;
use proconio::source::Readable;

fn main() {
    input! {
        N: usize,
    }

    let ans = (1..=N)
        .filter(|&x| !format!("{}{:o}", x, x).contains('7'))
        .count();
    println!("{}", ans);
}
