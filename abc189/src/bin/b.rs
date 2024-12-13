#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, IntoAsciiString};
use itertools::FoldWhile::{Continue, Done};
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
    let result = A
        .into_iter()
        .zip(1..=N)
        .fold_while(0, |total_alcohol, (sake, i)| {
            let alcohol = sake.ml * sake.alcohol;
            if total_alcohol + alcohol > limit {
                Done(i)
            } else {
                Continue(total_alcohol + alcohol)
            }
        });

    let ans = if let Done(i) = result { i } else { -1 };
    println!("{}", ans);
}
