use itertools::sorted;
use proconio::input;

fn main() {
    input!(mut a: [u32; 3]);
    let b = a[1];
    println!(
        "{}",
        if sorted(a).as_slice()[1] == b {
            "Yes"
        } else {
            "No"
        }
    )
    // a.sort();
    // println!("{}", if a[1] == b { "Yes" } else { "No" })
}
