#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, IntoAsciiString};
use itertools::Itertools;
use proconio::source::{Readable, Source};
use proconio::{derive_readable, input};
use std::io::BufRead;

trait AsSize {
    fn as_isize(&self) -> isize;
    fn as_usize(&self) -> usize;
}

macro_rules! impl_as_size {
    ( $t:ty ) => {
        impl AsSize for $t {
            fn as_isize(&self) -> isize {
                (*self).try_into().unwrap()
            }

            fn as_usize(&self) -> usize {
                (*self).try_into().unwrap()
            }
        }
    };
}

impl_as_size!(i8);
impl_as_size!(i16);
impl_as_size!(i32);
impl_as_size!(i64);
impl_as_size!(isize);
impl_as_size!(u8);
impl_as_size!(u16);
impl_as_size!(u32);
impl_as_size!(u64);
impl_as_size!(usize);

enum AsciiChars {}

impl Readable for AsciiChars {
    type Output = Vec<AsciiChar>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<AsciiChar> {
        let token = source.next_token_unwrap();
        token.into_ascii_string().unwrap().into()
    }
}

#[derive_readable]
struct Sake {
    ml: isize,
    alcohol: isize,
}

#[cfg(target_pointer_width = "64")]
fn main() {
    input! {
        N: isize, X: isize,
        A: [Sake; N]
    }
    let limit = X * 100;
    let ans = A
        .into_iter()
        .scan(0, |total_alcohol, sake| {
            *total_alcohol += sake.ml * sake.alcohol;
            Some(*total_alcohol)
        })
        .find_position(|total_alcohol| *total_alcohol > limit)
        .map_or(-1, |(i, _)| i.as_isize() + 1);

    println!("{}", ans);
}
