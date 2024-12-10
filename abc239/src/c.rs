#![allow(nonstandard_style)]

use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    input! { P1: Point, P2: Point };

    let exists = solve(&P1, &P2);
    let ans = if exists { "Yes" } else { "No" };
    println!("{}", ans);
}

/**
 * (a - c)^2 + (b - d)^2 == 5
 * (1, 4) // |a - c| == 1 && |b - d| == 2
 * (4, 1) // |a - c| == 2 && |b - d| == 1
 *
 * ((|P1.x - c| == 1 && |P1.y - d| == 2) || (|P1.x - c| == 2 && |P1.y - d| == 1))
 * &&
 * ((|P2.x - c| == 1 && |P2.y - d| == 2) || (|P2.x - c| == 2 && |P2.y - d| == 1))
 */
fn solve(P1: &Point, P2: &Point) -> bool {
    for c in (P1.x - 2)..=(P1.x + 2) {
        let ac_P1 = (P1.x - c).abs();
        let ac_P2 = (P2.x - c).abs();
        for d in (P1.y - 2)..=(P1.y + 2) {
            let bd_P1 = (P1.y - d).abs();
            let bd_P2 = (P2.y - d).abs();

            let exists_P1 = (ac_P1 == 1 && bd_P1 == 2) || (ac_P1 == 2 && bd_P1 == 1);
            let exists_P2 = (ac_P2 == 1 && bd_P2 == 2) || (ac_P2 == 2 && bd_P2 == 1);
            if exists_P1 && exists_P2 {
                return true;
            }
        }
    }

    false
}
