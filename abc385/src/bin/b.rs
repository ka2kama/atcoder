#![allow(unused, nonstandard_style)]

use crate::my_lib::my_iter::*;
use crate::my_lib::my_num::*;
use itertools::Itertools;
use maplit::hashset;
use nalgebra::coordinates::X;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

pub mod my_lib {
    pub mod my_iter {
        use crate::my_lib::my_iter::scan_left::ScanLeft;
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
                pub(crate) fn new(iter: I, state: St, f: F) -> Self {
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        H: isize, W: isize, X: Usize1, Y: Usize1,
        A: [Chars; H],
        T: Chars
    }

    let mut houses = hashset![];
    let (mut x, mut y) = (X, Y);
    for command in T {
        let (next_x, next_y) = match command {
            'U' => (x - 1, y),
            'D' => (x + 1, y),
            'L' => (x, y - 1),
            'R' => (x, y + 1),
            _ => unreachable!(),
        };

        match A[next_x][next_y] {
            '#' => {}
            ch => {
                (x, y) = (next_x, next_y);
                if ch == '@' {
                    houses.insert((x, y));
                }
            }
        }
    }

    println!("{} {} {}", x + 1, y + 1, houses.len());
}
