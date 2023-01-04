#![allow(unused, nonstandard_style)]

use std::fmt::format;
use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[derive(Debug, PartialEq)]
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn distance(&self, other: &Point2d) -> f64 {
        let d = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        d.sqrt()
    }
}

#[fastout]
fn main() {
    input! { A: f64, B: f64, H: f64, M: f64 };

    // H時M分のときの時針の先端座標
    let point_H = {
        // H時M分のときの時針の角度 = H時0分のときの角度 + M分経ったときの角度
        // 時針は1時間で30度進む (360度 / 12時間)
        // 時針は1分で0.5度進む  (360度 / 720分)
        let degree_H = H * 30. + M * 0.5;
        let radian_H = degree_H.to_radians();

        // 時針の長さを半径とする円を考えて、先端座標を求める
        Point2d {
            x: A * radian_H.cos(), // 半径 * (x / 半径) = x
            y: A * radian_H.sin(), // 半径 * (y / 半径) = y
        }
    };

    // M分のときの分針の先端座標
    let point_M = {
        // M分のときの分針の角度
        // 分針は1分で6度進む (360度 / 60分)
        let degree_M = M * 6.;
        let radian_M = degree_M.to_radians();
        Point2d {
            x: B * radian_M.cos(),
            y: B * radian_M.sin(),
        }
    };

    let ans = point_H.distance(&point_M);
    println!("{:.15}", ans);
}
