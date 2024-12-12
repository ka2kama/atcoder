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

fn main() {
    input! {
        N: usize,
        A: AsciiChars,
    }

    let app = App { N, A };
    app.run();
}

struct App {
    N: usize,
    A: Vec<AsciiChar>,
}

impl App {
    #[fastout]
    fn run(self) {
        let ans = "";
        println!("{}", ans);
    }
}
