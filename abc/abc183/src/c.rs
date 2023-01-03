#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { N: usize, K: usize, T: [[usize; N]; N]};
    let ans = solve(N, K, &T);
    println!("{}", ans);
}

fn solve(N: usize, K: usize, T: &[Vec<usize>]) -> usize {
    (1..N)
        .permutations(N - 1)
        .map(|route| {
            let (last, acc) = route
                .into_iter()
                .fold((0, 0), |(prev, acc), next| (next, acc + T[prev][next]));
            acc + T[last][0]
        })
        .filter(|&s| s == K)
        .count()
}
