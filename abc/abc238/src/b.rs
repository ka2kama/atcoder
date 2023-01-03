#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { H: usize, W: usize,  A: [[usize; W]; H] };
    let B = transpose(A);
    for row in B {
        println!("{}", row.into_iter().join(" "));
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
