#![allow(nonstandard_style)]

use proconio::input;

const IS_PRIMES: [bool; 201] = [
    false, false, true, true, false, true, false, true, false, false, false, true, false, true,
    false, false, false, true, false, true, false, false, false, true, false, false, false, false,
    false, true, false, true, false, false, false, false, false, true, false, false, false, true,
    false, true, false, false, false, true, false, false, false, false, false, true, false, false,
    false, false, false, true, false, true, false, false, false, false, false, true, false, false,
    false, true, false, true, false, false, false, false, false, true, false, false, false, true,
    false, false, false, false, false, true, false, false, false, false, false, false, false, true,
    false, false, false, true, false, true, false, false, false, true, false, true, false, false,
    false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, false, false, false, true, false, false, false, false, false, true, false,
    true, false, false, false, false, false, false, false, false, false, true, false, true, false,
    false, false, false, false, true, false, false, false, false, false, true, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    true, false, false, false, false, false, false, false, false, false, true, false, true, false,
    false, false, true, false, true, false,
];

#[derive(Debug)]
enum Player {
    Takahashi,
    Aoki,
}

fn main() {
    input! { A: usize, B: usize, C: usize, D: usize };
    let winner = solve(A, B, C, D);
    let ans = match winner {
        Player::Takahashi => "Takahashi",
        Player::Aoki => "Aoki",
    };
    println!("{}", ans);
}

fn solve(A: usize, B: usize, C: usize, D: usize) -> Player {
    for p in A..=B {
        if (p + C..=p + D).all(|x| !IS_PRIMES[x]) {
            return Player::Takahashi;
        }
    }

    Player::Aoki
}
