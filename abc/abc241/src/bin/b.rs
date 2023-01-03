#![allow(nonstandard_style)]

use im_rc::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, M: usize, A: [i64; N], B: [i64; M] };
    let ans = if solve(A, B) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn solve(A: Vec<i64>, B: Vec<i64>) -> bool {
    let mut map = HashMap::new();
    for &a in &A {
        *map.entry(a).or_insert(0_i64) += 1;
    }

    for b in &B {
        if let Some(cnt) = map.get(b) {
            if *cnt == 0 {
                return false;
            } else {
                map[b] -= 1;
            }
        } else {
            return false;
        }
    }
    true
}
