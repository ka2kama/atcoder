#![allow(nonstandard_style)]

use im_rc::hashset;
use itertools::enumerate;
use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, S: [Chars; N] };

    let ans = if solve(N, S) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn solve(N: usize, S: Vec<Vec<char>>) -> bool {
    let set = vec![
        // 6
        vec!['#', '#', '#', '#', '#', '#'],
        // 5
        vec!['.', '#', '#', '#', '#', '#'],
        vec!['#', '#', '#', '#', '#', '.'],
        // 4
        vec!['#', '#', '#', '#', '.', '#'],
        vec!['#', '#', '#', '#', '.', '.'],
        vec!['#', '.', '#', '#', '#', '#'],
        vec!['.', '.', '#', '#', '#', '#'],
        vec!['.', '#', '#', '#', '#', '.'],
        // 3
        vec!['#', '#', '#', '.', '#', '#'],
        vec!['#', '#', '#', '.', '#', '.'],
        vec!['.', '#', '#', '#', '.', '#'],
        vec!['#', '#', '.', '#', '#', '#'],
        vec!['#', '.', '.', '#', '#', '#'],
        vec!['#', '.', '#', '#', '#', '.'],
        // 2
        vec!['#', '#', '.', '.', '#', '#'],
        vec!['#', '#', '.', '#', '#', '.'],
        vec!['#', '#', '.', '#', '.', '#'],
        vec!['#', '.', '#', '#', '.', '#'],
        vec!['.', '#', '#', '.', '#', '#'],
    ];

    // 横
    for s in &S {
        for i in 0..N - 5 {
            for xs in &set {
                if itertools::equal(xs, &s[i..i + 6]) {
                    return true;
                }
            }
        }
    }

    // 縦
    let mut v = vec![vec!['0'; N]; N];
    for i in 0..N {
        for j in 0..N {
            v[j][i] = S[i][j];
        }
    }
    for s in &v {
        for i in 0..N - 5 {
            for xs in &set {
                if itertools::equal(xs, &s[i..i + 6]) {
                    return true;
                }
            }
        }
    }

    // 斜め
    let mut v = Vec::new();
    for i in 0..N - 5 {
        for j in 0..N - 5 {
            let mut slash = Vec::new();
            let mut k = 0;
            while i + k < N && j + k < N {
                slash.push(S[i + k][j + k]);
                k += 1;
            }
            v.push(slash);
        }
    }

    for s in &v {
        for i in 0..N - 5 {
            if s.len() < i + 6 {
                continue;
            }
            for xs in &set {
                if itertools::equal(xs, &s[i..i + 6]) {
                    return true;
                }
            }
        }
    }

    false
}
