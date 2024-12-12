#![allow(unused, nonstandard_style)]

use proconio::input;
use proconio::source::Readable;

fn main() {
    input! {
        N: usize, W: usize
    }

    println!("{}", N / W)
}
