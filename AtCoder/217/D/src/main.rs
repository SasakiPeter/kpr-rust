use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    // これはBTreeSetでいけちゃうぞこれ
    // input!(l: u32, q: u32);
    input! {
        l:u32,
        _q:u32,
        query: [(u8, u32); _q]
    }
    let mut s: BTreeSet<u32> = BTreeSet::new();
    s.insert(0);
    s.insert(l);
    // for _ in 0..q {
    for (c, x) in query {
        // input!(c: u8, x: u32);
        if c == 1 {
            s.insert(x);
        } else {
            let max = *s.range(x..).next().unwrap();
            let min = *s.range(..x).next_back().unwrap();
            println!("{}", max - min);
            // if let Some(&max) = s.range(x..).next() {
            //     if let Some(&min) = s.range(..x).next_back() {
            //         println!("{}", max - min);
            //     }
            // }
        }
    }
}
