#![allow(unused, nonstandard_style)]

use crate::my_lib::my_iter::*;
use crate::my_lib::my_num::*;
use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::marker::Chars;
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

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        K: isize,
        S: Chars,
        T: Chars
    }

    let ans = if check(S, T) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn check(S: Vec<char>, T: Vec<char>) -> bool {
    if S == T {
        return true;
    }

    // check: S中の文字を1つ選び、別の1つの文字に変更する。
    if S.len() == T.len() {
        let diff_cnt = S
            .into_iter()
            .zip(T.into_iter())
            .filter(|(a, b)| a != b)
            .count();
        return diff_cnt <= 1;
    }

    let (longer, shorter) = if S.len() > T.len() { (S, T) } else { (T, S) };
    if longer.len() - shorter.len() > 1 {
        return false;
    }
    // 同じ文字列 + 末尾1文字
    if &longer[..=shorter.len() - 1] == &shorter {
        return true;
    }
    let mut op_cnt = 0;
    for (i, &ch) in shorter.iter().enumerate() {
        while i + op_cnt < longer.len() {
            if ch == longer[i + op_cnt] {
                break;
            }
            op_cnt += 1;
        }
    }
    op_cnt == 1
}
