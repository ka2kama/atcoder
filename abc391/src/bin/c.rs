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
use std::io::Read;
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

         #[inline]
         fn product_i64<A>(self) -> i64
         where
            A: AsNumber,
            Self: Sized + Iterator<Item = A>,
         {
            self.map(|x| x.as_i64()).product()
         }

         #[inline]
         fn count_as_i64<A>(self) -> i64
         where
            A: AsNumber,
            Self: Sized + Iterator<Item = A>,
         {
            self.count().as_i64()
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct P(usize);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct H(usize);

#[cfg(target_pointer_width = "64")]
fn main() {
   let stdin = std::io::stdin();
   let mut s = String::new();
   stdin.read_line(&mut s).unwrap();
   let (N, Q) = s.split_whitespace().collect_tuple().unwrap();
   let N: usize = N.parse().unwrap();
   let Q: usize = Q.parse().unwrap();

   let mut pegion = HashMap::new();
   for i in 1..=N {
      pegion.insert(P(i), H(i));
   }
   let mut nest = vec![1i64; N + 1];
   nest[0] = 0;
   let mut multiple = 0i64;
   for i in 0..Q {
      let mut q = String::new();
      stdin.read_line(&mut q).unwrap();
      let q = q.trim_end();
      match q {
         "2" => {
            println!("{}", multiple);
         }
         _ => {
            let (_, p, h) = q.split_whitespace().collect_tuple().unwrap();
            let p = P(p.parse::<usize>().unwrap());
            let dist_h = H(h.parse::<usize>().unwrap());
            let current_h = pegion[&p];
            if nest[current_h.0] == 2 {
               multiple -= 1;
            }
            if nest[dist_h.0] == 1 {
               multiple += 1;
            }
            nest[dist_h.0] += 1;
            nest[current_h.0] = (nest[current_h.0] - 1).max(0);
            pegion.insert(p, dist_h);
         }
      }
   }
}
