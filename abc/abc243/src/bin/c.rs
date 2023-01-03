#![allow(nonstandard_style)]

use proconio::derive_readable;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

enum Direction {
    Left,
    Right,
}

#[fastout]
fn main() {
    input! { N:usize, A: [Point; N], S: Chars };
    let ans = if solve(A, S) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn solve(A: Vec<Point>, S: Vec<char>) -> bool {
    let mut map: HashMap<i64, (Option<&Point>, Option<&Point>)> = HashMap::new();
    for (p1, d) in A.iter().zip(S.iter().map(|&s| match s {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!(),
    })) {
        let pair = match map.get(&p1.y) {
            Some(&(left, right)) => match d {
                Direction::Left => match right {
                    Some(p2) => (left, Some(if p1.x > p2.x { p1 } else { p2 })),
                    None => (left, Some(p1)),
                },
                Direction::Right => match left {
                    Some(p2) => (Some(if p1.x < p2.x { p1 } else { p2 }), right),
                    None => (Some(p1), right),
                },
            },
            None => match d {
                Direction::Left => (None, Some(p1)),
                Direction::Right => (Some(p1), None),
            },
        };
        map.insert(p1.y, pair);
    }

    map.into_iter().map(|(_, p)| p).any(|p| match p {
        (Some(left), Some(right)) => left.x < right.x,
        _ => false,
    })
}
