#![allow(unused, nonstandard_style)]

use crate::my_lib::*;
use ascii::{AsciiChar, AsciiString};
use itertools::Itertools;
use maplit::{hashmap, hashset};
use proconio::{derive_readable, input};
use std::collections::*;
use std::mem;

mod my_lib {
    use ascii::{AsciiChar, IntoAsciiString};
    use proconio::source::{Readable, Source};
    use std::io::BufRead;
    use std::iter::Rev;

    pub struct SimpleScan<I, St, F> {
        iter: I,
        state: St,
        f: F,
        started: bool,
    }

    impl<I, St, F> SimpleScan<I, St, F> {
        fn new(iter: I, initial_state: St, f: F) -> SimpleScan<I, St, F> {
            SimpleScan {
                iter,
                state: initial_state,
                f,
                started: false,
            }
        }

        fn snap_shot(&self) -> St
        where
            St: Clone,
        {
            self.state.clone()
        }
    }

    impl<I, St, F, A> Iterator for SimpleScan<I, St, F>
    where
        St: Clone,
        I: Iterator<Item = A>,
        F: FnMut(&mut St, A),
    {
        type Item = St;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.started {
                self.started = true;
                Some(self.snap_shot())
            } else if let Some(next) = self.iter.next() {
                (self.f)(&mut self.state, next);
                Some(self.snap_shot())
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, self.iter.size_hint().1)
        }
    }

    pub trait SimpleScanIterator: Iterator {
        #[inline]
        fn scan_left<St, F>(self, initial_state: St, f: F) -> SimpleScan<Self, St, F>
        where
            Self: Sized,
            F: FnMut(&mut St, Self::Item),
        {
            SimpleScan::new(self, initial_state, f)
        }

        #[inline]
        fn scan_right<St, F>(self, initial_state: St, f: F) -> SimpleScan<Rev<Self>, St, F>
        where
            Self: Sized + DoubleEndedIterator,
            F: FnMut(&mut St, Self::Item),
        {
            self.rev().scan_left(initial_state, f)
        }
    }

    impl<I: Iterator> SimpleScanIterator for I {}

    pub trait AsSize {
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

    pub enum AsciiChars {}

    impl Readable for AsciiChars {
        type Output = Vec<AsciiChar>;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<AsciiChar> {
            let token = source.next_token_unwrap();
            token.into_ascii_string().unwrap().into()
        }
    }
}

#[cfg(target_pointer_width = "64")]
fn main() {
    input! {
        N: isize,
    }
    let ans = "";
    println!("{}", ans);
}
