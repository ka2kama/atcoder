#![allow(unused, nonstandard_style)]

use crate::my_lib::my_iter::*;
use crate::my_lib::my_num::*;
use crate::my_lib::*;
use indexmap::{indexmap, indexset};
use itertools::Itertools;
use maplit::{hashmap, hashset};
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};
use std::collections::*;
use std::mem;

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        N: isize,
        X: [(isize, isize); N]
    }
    let X: Vec<(isize, isize)> = X;

    let ans = X.into_iter().flat_map(|(a, b)| a..=b).sum_isize();
    println!("{}", ans);
}

pub mod my_lib {
    pub mod my_iter {
        use crate::my_lib::my_iter::scan_left::ScanLeft;
        use crate::my_lib::my_num::AsSize;
        use std::iter::Rev;

        pub trait MyIterator: Iterator {
            #[inline]
            fn scan_left<St, F>(self, initial_state: St, f: F) -> ScanLeft<Self, St, F>
            where
                Self: Sized,
                St: Clone,
                F: FnMut(&mut St, Self::Item),
            {
                ScanLeft::new(self, initial_state, f)
            }

            #[inline]
            fn scan_right<St, F>(self, initial_state: St, f: F) -> ScanLeft<Rev<Self>, St, F>
            where
                Self: Sized + DoubleEndedIterator,
                St: Clone,
                F: FnMut(&mut St, Self::Item),
            {
                self.rev().scan_left(initial_state, f)
            }

            #[inline]
            fn collect_string(self) -> String
            where
                Self: Sized,
                String: FromIterator<Self::Item>,
            {
                self.collect()
            }

            #[inline]
            fn sum_isize<A>(self) -> isize
            where
                A: AsSize,
                Self: Sized + Iterator<Item = A>,
            {
                self.map(|x| x.as_isize()).sum()
            }
        }

        impl<I: Iterator> MyIterator for I {}

        mod scan_left {
            #[derive(Clone)]
            pub struct ScanLeft<I, St, F> {
                iter: I,
                state: St,
                f: F,
                started: bool,
            }

            impl<I, St, F> ScanLeft<I, St, F> {
                #[inline]
                pub(in crate::my_lib::my_iter) fn new(iter: I, state: St, f: F) -> Self {
                    Self {
                        iter,
                        state,
                        f,
                        started: false,
                    }
                }
            }

            impl<I, St, F> Iterator for ScanLeft<I, St, F>
            where
                St: Clone,
                I: Iterator,
                F: FnMut(&mut St, I::Item),
            {
                type Item = St;

                fn next(&mut self) -> Option<St> {
                    if !self.started {
                        // 初回実行
                        self.started = true;
                    } else {
                        // 2回目以降
                        let a = self.iter.next()?;
                        (self.f)(&mut self.state, a);
                    }
                    Some(self.state.clone())
                }
            }
        }
    }

    pub mod my_num {
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
    }
}
