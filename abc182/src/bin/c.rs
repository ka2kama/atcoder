#![allow(unused, nonstandard_style)]

use crate::my_lib::iter::*;
use crate::my_lib::models::*;
use crate::my_lib::num::*;
use crate::my_lib::*;
use indexmap::{indexmap, indexset};
use itertools::Itertools;
use num_integer::Integer;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};
use std::collections::*;
use std::mem;

pub mod my_lib {

    pub mod models {
        use proconio::derive_readable;

        #[derive_readable]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
        pub struct Point {
            pub x: i64,
            pub y: i64,
        }

        impl Point {
            pub fn new(x: i64, y: i64) -> Self {
                Self { x, y }
            }
        }

        #[derive_readable]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct PointF {
            pub x: f64,
            pub y: f64,
        }

        impl PointF {
            pub fn new(x: f64, y: f64) -> Self {
                Self { x, y }
            }
        }
    }

    pub mod iter {
        use crate::my_lib::iter::scan_left::ScanLeft;
        use crate::my_lib::num::AsNumber;
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
            fn sum_i64<A>(self) -> i64
            where
                A: AsNumber,
                Self: Sized + Iterator<Item = A>,
            {
                self.map(|x| x.as_i64()).sum()
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
                pub(in crate::my_lib::iter) fn new(iter: I, state: St, f: F) -> Self {
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

    pub mod num {
        pub trait AsNumber {
            fn as_i64(&self) -> i64;
            fn as_usize(&self) -> usize;
        }

        macro_rules! impl_as_number {
            ( $t:ty ) => {
                impl AsNumber for $t {
                    fn as_i64(&self) -> i64 {
                        (*self).try_into().unwrap()
                    }

                    fn as_usize(&self) -> usize {
                        (*self).try_into().unwrap()
                    }
                }
            };
        }

        impl_as_number!(i8);
        impl_as_number!(i16);
        impl_as_number!(i32);
        impl_as_number!(i64);
        impl_as_number!(isize);
        impl_as_number!(u8);
        impl_as_number!(u16);
        impl_as_number!(u32);
        impl_as_number!(u64);
        impl_as_number!(usize);
    }
}

fn solve(N: i64) -> i64 {
    let m = N % 3;
    if m == 0 {
        return 0;
    }

    let v = N
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect_vec();

    if m == 1 {
        if v.len() > 1 && v.iter().any(|x| *x % 3 == 1) {
            return 1;
        }

        if v.len() > 2 && v.iter().filter(|x| **x % 3 == 2).take(2).count() == 2 {
            return 2;
        }
    }

    if m == 2 {
        if v.len() > 1 && v.iter().any(|x| *x % 3 == 2) {
            return 1;
        }

        if v.len() > 2 && v.iter().filter(|x| **x % 3 == 1).take(2).count() == 2 {
            return 2;
        }
    }

    -1
}

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        N: i64,
    }

    let ans = solve(N);
    println!("{}", ans);
}
