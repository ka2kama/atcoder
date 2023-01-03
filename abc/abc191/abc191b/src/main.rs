#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { N: usize, X: usize, A: [usize; N] };
    println!("{}", A.into_iter().filter(|&x| x != X).join(" "));
}
