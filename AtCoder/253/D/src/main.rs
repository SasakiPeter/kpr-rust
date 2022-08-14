use num_integer::lcm;
use proconio::input;

fn main() {
    input!(n: u64, a: u64, b: u64);
    let sum_n = (1 + n) * n / 2;

    fn calc(n: u64, x: u64) -> u64 {
        let n_x = n / x;
        return x * (1 + n_x) * n_x / 2;
    }
    let sum_a: u64 = calc(n, a);
    let sum_b: u64 = calc(n, b);

    let sum_ab: u64 = calc(n, lcm(a, b));
    let ans = sum_n - sum_a - sum_b + sum_ab;
    println!("{}", ans);
    // println!("{}, {}, {}, {}, {}", ans, sum_n, sum_a, sum_b, sum_ab);
}
