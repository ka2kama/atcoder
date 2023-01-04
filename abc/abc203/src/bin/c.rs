#![allow(unused, nonstandard_style)]

use std::collections::VecDeque;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
#[derive(Debug)]
struct Friend {
    village: i64,
    money: i64,
}

#[fastout]
fn main() {
    input! { N: i64, K: i64, A: [Friend; N], }
    let mut friends: VecDeque<Friend> = A.into_iter().sorted_by_key(|f| f.village).collect();
    let ans = {
        let mut current_village = K;
        while !friends.is_empty() && friends[0].village <= current_village {
            let friend = friends.pop_front().unwrap();
            current_village += friend.money;
        }
        current_village
    };
    println!("{}", ans);
}
