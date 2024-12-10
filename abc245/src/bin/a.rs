#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { A: i64, B: i64, C: i64, D: i64 };

    let ans = solve(A, B, C, D);
    println!("{}", ans);
}

fn solve(A: i64, B: i64, C: i64, D: i64) -> String {
    let takahashi = "Takahashi".to_string();
    let aoki = "Aoki".to_string();

    if A < C {
        return takahashi;
    } else if A > C {
        return aoki;
    }

    if B < D {
        return takahashi;
    } else if B > D {
        return aoki;
    };

    takahashi
}
