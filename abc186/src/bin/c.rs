#![allow(unused, nonstandard_style)]

use std::collections::VecDeque;
use std::iter;

use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, }
    let ans = (1..=N)
        .filter(|&x| !x.to_string().contains('7') && !to_oct(x).contains('7'))
        .count();
    println!("{}", ans);
}

fn to_oct(x: usize) -> String {
    let mut d = VecDeque::new();
    let mut div = x;
    loop {
        d.push_front(div % 8);
        div /= 8;
        if div == 0 {
            break;
        }
    }
    d.iter().join("")
}
