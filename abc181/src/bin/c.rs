#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use nalgebra::Point2;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[fastout]
fn main() {
    input! { N: usize, A: [Point; N], }
    let mut ans = "No";
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                let (p1, p2, p3) = (&A[i], &A[j], &A[k]);
                if (p2.x - p1.x) * (p3.y - p1.y) == (p2.y - p1.y) * (p3.x - p1.x) {
                    ans = "Yes";
                }
            }
        }
    }
    println!("{}", ans);
}
