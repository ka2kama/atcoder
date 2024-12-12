#![allow(unused, nonstandard_style)]

use itertools::Itertools;
use proconio::{fastout, input};
use smallvec::SmallVec;

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let answers = (1..=N)
        .take_while(|&i| i * i <= N)
        .flat_map(|i| {
            let mut v = SmallVec::<[usize; 2]>::new();
            if N % i == 0 {
                v.push(i);
                if i != N / i {
                    v.push(N / i);
                }
            }
            v
        })
        .sorted_unstable();

    for ans in answers {
        println!("{}", ans);
    }
}
