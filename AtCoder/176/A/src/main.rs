use proconio::input;
fn main() {
    input!(n: u32, x: u32, t: u32);
    // // pythonみたいに、負の切り捨てで値が小さくならない
    // println!("{}", t * (-(-n / x)));
    // println!("{}", t * ((n - 1) / x + 1));
    println!("{}", t * ((n + x - 1) / x));
}
