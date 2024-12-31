#![allow(unused, nonstandard_style)]

use crate::my_lib::iter::*;
use crate::my_lib::models::*;
use crate::my_lib::num::*;
use crate::my_lib::*;
use indexmap::{indexmap, indexset};
use itertools::Itertools;
use maplit::{hashmap, hashset};
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Slope {
    Vertical,
    Horizontal,
    Positive { dx: i64, dy: i64 },
}

fn exist_collinear_among_n(mut A: Vec<Point>, n: i64) -> bool {
    if n < 2 {
        return false;
    }
    A.sort_unstable();
    A.dedup();
    let A = A.as_slice();
    let required_same_slope_cnt = n - 1;
    for pivot in A {
        let mut slope_cnt = hashmap! {};
        for p2 in A {
            if p2 == pivot {
                continue;
            }

            let dx = p2.x - pivot.x;
            let dy = p2.y - pivot.y;
            let slope = if dx == 0 {
                Slope::Vertical
            } else if dy == 0 {
                Slope::Horizontal
            } else {
                // to positive incline
                let (dx, dy) = if dx < 0 { (-dx, -dy) } else { (dx, dy) };

                // reduce to lowest terms
                let g = dx.gcd(&dy);
                let (dx, dy) = (dx / g, dy / g);

                Slope::Positive { dx, dy }
            };

            let cnt = slope_cnt.entry(slope).or_insert(0);
            *cnt += 1;
            if *cnt >= required_same_slope_cnt {
                return true;
            }
        }
    }

    false
}

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        N: i64,
        A: [Point; N],
    }

    let ans = if exist_collinear_among_n(A, 3) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
