use proconio::input;

fn main() {
    input! { k: i32, s: i32, }

    let ans = solve(k, s);
    println!("{}", ans);
}

fn solve(k: i32, n: i32) -> i32 {
    let mut cnt = 0;
    for x in 0..=k {
        for y in 0..=k {
            let z = n - x - y;
            if 0 <= z && z <= k {
                cnt += 1
            }
        }
    }
    cnt
}
