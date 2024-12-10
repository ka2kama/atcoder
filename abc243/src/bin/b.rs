#![allow(nonstandard_style)]

use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! { N: usize, A: [i64; N], B: [i64; N] };
    let (ans1, ans2) = solve(A, B);
    println!("{}", ans1);
    println!("{}", ans2);
}

fn solve(A: Vec<i64>, B: Vec<i64>) -> (usize, usize) {
    let a_set: HashSet<_> = A.iter().map(|&x| x).collect();
    let b_set: HashSet<_> = B.iter().map(|&x| x).collect();
    let a_i_set: HashSet<_> = A.iter().enumerate().map(|(i, &x)| (i, x)).collect();
    let b_i_set: HashSet<_> = B.iter().enumerate().map(|(i, &x)| (i, x)).collect();

    let intersection = a_set.intersection(&b_set).count();
    let intersection_i = a_i_set.intersection(&b_i_set).count();
    (intersection_i, intersection - intersection_i)
}
