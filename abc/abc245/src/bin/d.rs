#![allow(nonstandard_style)]

use std::collections::HashMap;
use std::io;

struct Poem {
    line: u32,
    score: u32,
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let vec: Vec<u32> = input
        .trim()
        .split_whitespace()
        .flat_map(|i| i.parse())
        .collect();

    let n = vec[0];
    let mut hash = HashMap::new();

    for (_, line) in (0..n).zip(1..) {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let vec: Vec<&str> = s.trim().split_whitespace().collect();
        let s = vec[0].to_string();
        let t: u32 = vec[1].parse().unwrap();
        hash.entry(s).or_insert(Poem { line, score: t });
    }

    let mut max = 0;
    let mut ans = 0;
    for (_, poem) in &hash {
        if max <= poem.score {
            max = poem.score;
            ans = poem.line;
        }
    }
    let ans = hash
        .values()
        .max_by_key(|p| p.score)
        .map(|p| p.line)
        .unwrap_or(0);
    println!("{}", ans);
}
