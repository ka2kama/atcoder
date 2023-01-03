#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { A: [usize; 10] };
    let first = A[0];
    let second = A[first];
    let third = A[second];
    println!("{}", third);
}
