#![allow(unused, nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize,  A: [i64; N], B: [i64; N] };
    let inner_product_of_a_and_b: i64 = A.iter().zip(&B).map(|(a, b)| a * b).sum();
    println!(
        "{}",
        if inner_product_of_a_and_b == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
