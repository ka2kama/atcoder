#![allow(nonstandard_style)]

use num_bigint::BigInt;
use num_rational::BigRational;
use proconio::input;
use std::ops::Div;

fn main() {
    input! { X: BigRational, };
    let ans = X.div(BigRational::from_integer(BigInt::from(10))).floor();
    println!("{:.0}", ans);
}
