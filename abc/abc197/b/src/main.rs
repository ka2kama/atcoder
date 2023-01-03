#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::iter;

fn main() {
    input! { H: usize, W: usize, X: Usize1, Y: Usize1, A: [Chars; H] };
    let horizontal_line = &A[X];
    let vertical_line: Vec<char> = A.iter().map(|row| row[Y]).collect();
    let left = (0..Y)
        .rev()
        .map(|col| horizontal_line[col])
        .take_while(|&c| c == '.')
        .count();
    let right = ((Y + 1)..W)
        .map(|col| horizontal_line[col])
        .take_while(|&c| c == '.')
        .count();
    let up = (0..X)
        .rev()
        .map(|row| vertical_line[row])
        .take_while(|&c| c == '.')
        .count();
    let down = ((X + 1)..H)
        .map(|row| vertical_line[row])
        .take_while(|&c| c == '.')
        .count();

    let ans = right + left + up + down + 1;
    println!("{}", ans);
}
