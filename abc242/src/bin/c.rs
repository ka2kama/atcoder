#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, };
    let ans = solve(N);
    println!("{}", ans);
}

fn solve(N: usize) -> usize {
    const Z: usize = 998244353;

    let mut v = vec![vec![0_usize; 10]; N + 1];
    v[2][1] = 2;
    v[2][9] = 2;
    for x in 2..=8 {
        v[2][x] = 3;
    }

    for i in 3..=N {
        v[i][1] = &v[i - 1][1] + &v[i - 1][2];
        v[i][9] = &v[i - 1][9] + &v[i - 1][8];
        for j in 2..=8 {
            v[i][j] = &v[i - 1][j - 1] + &v[i - 1][j] + &v[i - 1][j + 1];
            v[i][j] %= Z;
        }
    }

    let m: usize = v[N][1..].iter().sum();

    m % Z
}
