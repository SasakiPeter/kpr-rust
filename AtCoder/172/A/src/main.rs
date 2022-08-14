use proconio::input;
fn main() {
    input!(a: u32);
    println!("{}", a + a.pow(2) + a.pow(3));

    // let mut ans: u32 = 0;
    // for i in 1..4 {
    //     ans += a.pow(i);
    // }
    // println!("{}", ans);
}
