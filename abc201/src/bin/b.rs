#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[derive_readable]
struct Mountain {
    name: String,
    height: usize,
}

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [Mountain; N],
    }

    let mut it = A.iter();
    let (mut top, mut second) = (it.next().unwrap(), it.next().unwrap());
    if top.height < second.height {
        std::mem::swap(&mut top, &mut second);
    }

    for m in it {
        if m.height > second.height {
            second = m;
            if top.height < second.height {
                std::mem::swap(&mut top, &mut second);
            }
        }
    }

    println!("{}", second.name);
}
