#![allow(nonstandard_style)]

use either::{Either, Left, Right};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { V:i64, A: i64, B: i64, C: i64 }
    let mut rest = V;
    let ans = loop {
        match solve(rest, A, B, C) {
            Left(new_rest) => {
                rest = new_rest;
            }
            Right(name) => break name,
        }
    };
    println!("{}", ans);
}

fn solve(V: i64, A: i64, B: i64, C: i64) -> Either<i64, String> {
    if V < A {
        return Right("F".to_owned());
    }
    let rest = V - A;
    if rest < B {
        return Right("M".to_owned());
    }

    let rest = rest - B;
    if rest < C {
        return Right("T".to_owned());
    }

    Left(rest - C)
}
