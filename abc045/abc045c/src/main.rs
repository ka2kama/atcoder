use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! { s: Chars, }

    let ans = solve(&s);
    println!("{}", ans);
}

fn solve(s: &Vec<char>) -> i64 {
    let n = s.len();
    let s0 = to_i64(s[0]);
    let mut total = 0;
    for bit in 0..1 << (n - 1) {
        let mut sum = 0;
        let mut a = s0;
        for i in 0..n - 1 {
            if bit & (1 << i) != 0 {
                sum += a;
                a = 0;
            }
            a = a * 10 + to_i64(s[i + 1])
        }
        sum += a;
        total += sum;
    }

    total
}

fn to_i64(c: char) -> i64 {
    c.to_digit(10).unwrap() as i64
}
