#![allow(unused, nonstandard_style)]

use proconio::input;
use std::convert::TryFrom;

fn main() {
    input! { N: i64, };
    println!(
        "{}",
        if i32::try_from(N).is_ok() {
            "Yes"
        } else {
            "No"
        }
    );
}
