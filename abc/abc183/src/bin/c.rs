#![allow(unused, nonstandard_style)]

use std::iter;

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::{derive_readable, fastout, input};

#[fastout]
fn main() {
    input! { N: usize, K: usize, T: [[usize; N]; N]};
    let ans = solve(N, K, T);
    println!("{}", ans);
}

fn solve(N: usize, K: usize, T: Vec<Vec<usize>>) -> usize {
    let first_city = 1;

    // 中間経路の全組み合わせ
    let inter_routes = (2..=N).permutations(N - 1);

    // 各中間経路にSTARTとGOAL (どちらも都市1) を追加して完全な経路にする
    let routes = inter_routes.map(|inter_route| {
        let mut route = Vec::with_capacity(inter_route.len() + 2);
        route.push(first_city);
        route.extend(inter_route);
        route.push(first_city);
        route
    });

    // 経路ごとの合計移動時間算出
    let travel_times = routes.map(|route| {
        route
            .iter()
            .zip(route[1..].iter())
            .fold(0, |travel_time, (current_city, next_city)| {
                travel_time + T[current_city - 1][next_city - 1]
            })
    });

    // 合計移動時間がちょうどKになる経路の数
    travel_times.filter(|&s| s == K).count()
}
