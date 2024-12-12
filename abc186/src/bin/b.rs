#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::input;
use proconio::source::Readable;

fn main() {
    input! {
        H: usize, W: usize,
        A: [[u64; W]; H],
    }

    let v = A.into_iter().flatten().collect_vec();
    let min = *v.iter().min().unwrap();
    let ans: u64 = v.into_iter().map(|x| x - min).sum();
    println!("{}", ans);
}
