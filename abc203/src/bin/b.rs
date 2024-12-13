#![allow(unused, nonstandard_style)]

use proconio::input;

fn main() {
    input! {
        N: usize, K: usize,
    }

    let k_sum: usize = (1..=K).sum();
    let ans: usize = (1..=N).map(|i| i * 100 * K + k_sum).sum();
    println!("{}", ans);
}
