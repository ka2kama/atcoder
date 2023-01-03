#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, X: usize, A: [(usize, usize); N]};
    let ans = if solve(N, X, A) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn solve(N: usize, X: usize, A: Vec<(usize, usize)>) -> bool {
    let mut dp = vec![vec![false; X + 1]; N + 1];
    dp[0][0] = true;
    for n in 1..=N {
        let (a, b) = A[n - 1];
        for x in 0..=X {
            dp[n][x] = (x >= a && dp[n - 1][x - a]) || (x >= b && dp[n - 1][x - b]);
        }
    }

    dp[N][X]
}
