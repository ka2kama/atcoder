#![allow(unused, nonstandard_style)]

use crate::my_lib::my_iter::*;
use crate::my_lib::my_num::*;
use amplify::confinement::Collection;
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
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

fn calc_max_buildings(building_indices: IndexSet<isize>) -> isize {
    if building_indices.len() < 2 {
        return building_indices.len().as_isize();
    }

    let max_interval = *building_indices.last().unwrap() - *building_indices.first().unwrap();
    let start_and_intervals = building_indices
        .iter()
        .flat_map(|&i| (1..=max_interval).map(move |interval| (i, interval)));

    let mut max_buildings = isize::MIN;
    for &start_index in &building_indices {
        for interval in 1..=max_interval {
            let buildings = count_by_interval(start_index, interval, &building_indices);
            max_buildings = max_buildings.max(buildings);
        }
    }
    max_buildings
}

fn count_by_interval(start: isize, interval: isize, building_indices: &IndexSet<isize>) -> isize {
    let mut current = start;
    let mut count = 0;
    while building_indices.contains(&current) {
        count += 1;
        current += interval;
    }
    count
}

#[cfg(target_pointer_width = "64")]
#[fastout]
fn main() {
    input! {
        N: isize,
        H: [isize; N],
    }

    let mut im = IndexMap::new();
    for (i, &h) in H.iter().enumerate() {
        im.entry(h)
            .or_insert_with(IndexSet::new)
            .insert(i.as_isize());
    }

    let ans = im
        .into_iter()
        .map(|(_, v)| calc_max_buildings(v))
        .max()
        .unwrap();
    println!("{}", ans);
}
