#![allow(nonstandard_style)]

use num_bigint::BigUint;
use num_traits::One;
use proconio::input;

fn main() {
    input! { N: u64, };
    let n = BigUint::from(N);
    let mut a = BigUint::one();
    let b = n.clone() * n.clone();
    for _ in 1..=N {
        if a > b {
            break;
        }
        a = a * BigUint::from(2_u64);
    }
    println!("{}", if a > b { "Yes" } else { "No" });
}
