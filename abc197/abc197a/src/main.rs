#![allow(nonstandard_style)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! { S: Chars, }
    println!("{}{}{}", S[1], S[2], S[0]);
}
