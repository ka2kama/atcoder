#![allow(nonstandard_style)]

use proconio::input;
use proconio::marker::Chars;
use std::char;
use std::collections::HashSet;

fn main() {
    input! { S: Chars, };

    let ans = solve(S);
    println!("{}", ans);
}

fn solve(S: Vec<char>) -> usize {
    let mut required: HashSet<char> = HashSet::new();
    let mut must_not_be: HashSet<char> = HashSet::new();
    for (i, &c) in S.iter().enumerate() {
        let d = char::from_digit(i as u32, 10).unwrap();
        if c == 'o' {
            required.insert(d);
        } else if c == 'x' {
            must_not_be.insert(d);
        }
    }
    let required = required;
    let must_not_be = must_not_be;

    if required.len() > 4 || must_not_be.len() == 10 {
        return 0;
    }

    (0..=9999)
        .map(|d| format!("{:04}", d))
        .map(|passwd| passwd.chars().collect::<HashSet<_>>())
        .filter(|used_numbers| {
            used_numbers.is_superset(&required)
                && used_numbers.iter().all(|d| !must_not_be.contains(d))
        })
        .count()
}
