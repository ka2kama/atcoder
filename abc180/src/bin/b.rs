#![allow(unused, nonstandard_style)]

use az::Az;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [isize; N], }

    let mut ans_m = 0usize;
    let mut ans_e_tmp = 0usize;
    let mut ans_c = 0usize;
    for x in A {
        let x_abs = x.unsigned_abs();
        ans_m += x_abs;
        ans_e_tmp += x_abs.pow(2);
        ans_c = ans_c.max(x_abs);
    }
    let ans_e = ans_e_tmp.az::<f64>().sqrt();

    println!("{}", ans_m);
    println!("{:.15}", ans_e);
    println!("{}", ans_c);
}
