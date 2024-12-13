#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, IntoAsciiString};
use proconio::source::{Readable, Source};
use proconio::{derive_readable, input};
use std::io::BufRead;

trait AsIUsize {
    fn as_isize(&self) -> isize;
    fn as_usize(&self) -> usize;
}

impl AsIUsize for isize {
    fn as_isize(&self) -> isize {
        *self
    }

    fn as_usize(&self) -> usize {
        (*self).try_into().unwrap()
    }
}

impl AsIUsize for usize {
    fn as_isize(&self) -> isize {
        (*self).try_into().unwrap()
    }

    fn as_usize(&self) -> usize {
        *self
    }
}

enum AsciiChars {}

impl Readable for AsciiChars {
    type Output = Vec<AsciiChar>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<AsciiChar> {
        let token = source.next_token_unwrap();
        token.into_ascii_string().unwrap().into()
    }
}

#[derive_readable]
struct Friend {
    village: isize,
    money: isize,
}

#[cfg(target_pointer_width = "64")]
fn main() {
    input! {
        N: isize, K: isize,
        mut A: [Friend; N],
    }

    A.sort_unstable_by_key(|f| f.village);

    let mut current_village = 0;
    let mut remaining_money = K;
    for friend in A {
        let move_cost = friend.village - current_village;
        if move_cost > remaining_money {
            break;
        }
        current_village = friend.village;
        remaining_money = remaining_money - move_cost + friend.money;
    }

    let last_village = current_village + remaining_money;
    println!("{}", last_village);
}
