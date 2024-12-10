#![allow(nonstandard_style)]

use maplit::hashset;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, K: i64, A: [i64; N], B: [i64; N]};
    let ans = if solve(K, A, B) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn solve(K: i64, A: Vec<i64>, B: Vec<i64>) -> bool {
    let mut prev = hashset![A[0], B[0]];
    for (a, b) in A.into_iter().skip(1).zip(B.into_iter().skip(1)) {
        let mut next = hashset![];
        for &p in &prev {
            if (a - p).abs() <= K {
                next.insert(a);
            }
            if (b - p).abs() <= K {
                next.insert(b);
            }
        }
        if next.is_empty() {
            return false;
        }
        prev = next;
    }

    true
}
