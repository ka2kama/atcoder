#![allow(unused, nonstandard_style)]

use ascii::{AsciiChar, AsciiString, IntoAsciiString};
use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::marker::{Chars, Usize1};
use proconio::source::{Readable, Source};
use proconio::{derive_readable, fastout, input};
use std::io::BufRead;

enum AsciiChars {}

impl Readable for AsciiChars {
    type Output = Vec<AsciiChar>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<AsciiChar> {
        let token = source.next_token_unwrap();
        token.into_ascii_string().unwrap().into()
    }
}

struct App {
    H: usize,
    W: usize,
    X: usize,
    Y: usize,
    A: Vec<Vec<AsciiChar>>,
}

impl App {
    #[fastout]
    fn run(self) {
        let left = (0..self.Y).rev().map(|y| (self.X, y));
        let right = (self.Y + 1..self.W).map(|y| (self.X, y));
        let top = (0..self.X).rev().map(|x| (x, self.Y));
        let bottom = (self.X + 1..self.H).map(|x| (x, self.Y));
        let ans = self.open_count(left)
            + self.open_count(right)
            + self.open_count(top)
            + self.open_count(bottom)
            + 1; // A[X][Y]

        println!("{}", ans);
    }

    fn open_count<I: IntoIterator<Item = (usize, usize)>>(&self, it: I) -> usize {
        it.into_iter()
            .take_while(|&(x, y)| self.A[x][y] != AsciiChar::Hash)
            .count()
    }
}

fn main() {
    input! {
        H: usize, W: usize, X: Usize1, Y: Usize1,
        A: [AsciiChars; H],
    }

    let app = App { H, W, X, Y, A };
    app.run();
}
