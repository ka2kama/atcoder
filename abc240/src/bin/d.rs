#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::{fastout, input};
use std::sync::Mutex;

#[fastout]
fn main() {
    input! { N: usize, A: [usize; N]};
    let mut v = (vec![], Mutex::new(vec![]));
    let mut x = A[0];
    v.1 = Mutex::new(vec![x]);
    println!("1");
    let mut i = 1;
    while i < A.len() {
        let a = A[i];
        if a == x {
            v.1.lock().unwrap().push(a);
        } else {
            x = a;
            v.0 = vec![v.0, v.1.lock().unwrap().to_vec()].into_iter().concat();
            v.1 = Mutex::new(vec![x]);
        }
        if a == v.1.lock().unwrap().len() {
            v.1 = Mutex::new(vec![]);
            if v.0.is_empty() {
                println!("0");
                if i == A.len() - 1 {
                    break;
                }
                x = A[i + 1];
                println!("1");
                i += 2;
                continue;
            } else {
                x = v.0[v.0.len() - 1];
            }
        }
        println!("{}", v.0.len() + v.1.lock().unwrap().len());

        i += 1;
    }
}
