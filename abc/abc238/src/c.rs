#![allow(nonstandard_style)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! { S: Chars, };
    let ans = solve(S);
    println!("{}", if ans { "Yes" } else { "No" });
}

fn solve(S: Vec<char>) -> bool {
    let left_a = S.iter().take_while(|&c| *c == 'a').count();
    // aしかなかった場合
    if left_a == S.len() {
        return true;
    }

    let middle_and_right = &S[left_a..];
    let right_a = middle_and_right
        .iter()
        .rev()
        .take_while(|&c| *c == 'a')
        .count();
    let middle = &middle_and_right[..middle_and_right.len() - right_a];

    if !middle
        .iter()
        .zip(middle.iter().rev())
        .all(|(&l, &r)| l == r)
    {
        return false;
    }

    left_a <= right_a
}
