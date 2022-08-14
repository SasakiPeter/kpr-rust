use proconio::input;
// const PI: f64 = 3.141592653589793;
fn main() {
    input!(r: f64);
    // println!("{}", 2. * PI * r);
    // println!("{}", PI);
    // println!("{}", std::f64::consts::PI);
    println!("{}", 2. * std::f64::consts::PI * r);
    // println!("{:.7}", 2. * std::f64::consts::PI * r);
}
