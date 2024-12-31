#![allow(unused, nonstandard_style)]

use crate::my_lib::my_iter::*;
use crate::my_lib::my_num::*;
use crate::my_lib::*;
use indexmap::{indexmap, indexset, IndexMap};
use itertools::Itertools;
use maplit::{hashmap, hashset};
use num_integer::Integer;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};
use std::collections::hash_map::IntoIter;
use std::collections::*;
use std::mem;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Slope {
    Vertical,
    Horizontal,
    Inclined { dx: isize, dy: isize },
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[derive_readable]
struct Point {
    x: isize,
    y: isize,
}

fn to_collinear_points(pivot: &Point, A: &[Point]) -> HashMap<Slope, Vec<Point>> {
    A.iter()
        .filter(|p2| **p2 != *pivot)
        .map(|p2| (calc_slope(pivot, p2), p2.clone()))
        .into_group_map()
}

fn calc_slope(p1: &Point, p2: &Point) -> Slope {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    if dx == 0 {
        Slope::Vertical
    } else if dy == 0 {
        Slope::Horizontal
    } else {
        let g = dx.gcd(&dy);

        let dx_reduced = dx / g;
        let dy_reduced = dy / g;

        // Normalize sign: we can ensure dx_reduced is always positive
        // or if dx_reduced < 0, then multiply both by -1
        let (dx_norm, dy_norm) = if dx_reduced < 0 {
            (-dx_reduced, -dy_reduced)
        } else {
            (dx_reduced, dy_reduced)
        };

        Slope::Inclined {
            dx: dx_norm,
            dy: dy_norm,
        }
    }
}

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        N: isize,
        mut A: [Point; N],
    }
    A.sort_unstable();
    let A: Vec<Point> = A;

    let has_collinear = A
        .iter()
        .flat_map(|p| to_collinear_points(p, &A).into_values())
        .any(|v| v.len() >= 2);

    let ans = if has_collinear { "Yes" } else { "No" };
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
