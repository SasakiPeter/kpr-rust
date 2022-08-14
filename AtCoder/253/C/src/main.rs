use proconio::{fastout, input};
use std::collections::BTreeMap;

// #[fastout]
// fn main() {
//     input!(q: u32);
//     let mut ms: LikeMultiSet::new();
//     for _ in 0..q {
//         input!(n: u8);
//         if n == 1 {
//             input!(x: u64);
//             ms.insert(x);
//         } else if n == 2 {
//             input!(x: u64, c: i64);
//             ms.remove(x, c);
//         // こういう書き方するときは、xがkeyに存在しているか判定しないといけない
//         // *btm.entry(x).or_insert(0) -= if btm[&x] < c { btm[&x] } else { c };
//         // if btm[&x] == 0 {
//         //     btm.remove(&x);
//         // }
//         } else {
//             let ans = ms.get_diff();
//             println!("{}", ans);
//         }
//     }
// }

// struct LikeMultiSet {
//     map: BTreeMap<u64, i64>,
// }

// trait Collection {
//     fn new() -> LikeMultiSet;
//     fn insert(&mut self, x: u64) -> ();
//     fn erase(&mut self, x: u64, c: i64) -> ();
//     fn get_diff(&self) -> u64;
// }

// impl Collection for LikeMultiSet {
//     fn new() -> <Self as LikeMultiSet>::LikeMultiSet {
//         LikeMultiSet {
//             map: BTreeMap::new(),
//         }
//     }
//     fn insert(&mut self, x: u64) -> () {
//         *self.map.entry(x).or_insert(0) += 1
//     }

//     fn erase(&mut self, x: u64, c: i64) -> () {
//         let val = self.map.entry(x).or_insert(0);
//         *val -= c;
//         if *val <= 0 {
//             self.map.remove(&x);
//         }
//     }

//     fn get_diff(&self) -> u64 {
//         return self.map.iter().next_back().unwrap().0 - self.map.iter().next().unwrap().0;
//     }
// }

#[fastout]
fn main() {
    input!(q: u32);
    let mut btm: BTreeMap<u64, i64> = BTreeMap::new();
    for _ in 0..q {
        input!(n: u8);
        if n == 1 {
            input!(x: u64);
            *btm.entry(x).or_insert(0) += 1;
        } else if n == 2 {
            input!(x: u64, c: i64);

            let val = btm.entry(x).or_insert(0);
            *val -= c;
            if *val <= 0 {
                btm.remove(&x);
            }
        // こういう書き方するときは、xがkeyに存在しているか判定しないといけない
        // *btm.entry(x).or_insert(0) -= if btm[&x] < c { btm[&x] } else { c };
        // if btm[&x] == 0 {
        //     btm.remove(&x);
        // }
        } else {
            let ans = btm.iter().next_back().unwrap().0 - btm.iter().next().unwrap().0;
            println!("{}", ans);
        }
    }
}

// use proconio::{fastout, input};
// use std::collections::{BTreeSet, HashMap};

// // lower_bondが必要ないから、BTreeMapで事足りる

// #[fastout]
// fn main() {
//     input!(q: u32);
//     let mut ms = MultiSet::new();
//     for _ in 0..q {
//         input!(n: u8);
//         if n == 1 {
//             input!(x: u64);
//             ms.insert(x);
//         } else if n == 2 {
//             input!(x: u64, c: u64);
//             ms.erase(x, c);
//         } else {
//             println!("{}", ms.get_diff());
//         }
//     }
// }

// // BTreeSetはSortedSet
// // BTreeMapはSortedDict

// #[derive(Debug)]
// struct MultiSet {
//     s: BTreeSet<u64>,
//     d: HashMap<u64, u64>,
//     // 高速にある値以上の値の個数がわからなそう
// }

// // 実はtrait使わなくても、直接メソッド定義できるやん
// impl MultiSet {
//     fn new() -> MultiSet {
//         MultiSet {
//             s: BTreeSet::new(),
//             d: HashMap::new(),
//         }
//     }

//     fn insert(&mut self, x: u64) -> () {
//         self.s.insert(x);
//         *self.d.entry(x).or_insert(0) += 1
//         // let cnt = *self.d.entry(x).or_insert(0);
//         // self.d.insert(x, cnt + 1);
//     }

//     fn erase(&mut self, x: u64, c: u64) -> () {
//         if self.s.get(&x) == Some(&x) {
//             // *self.d.entry(x).or_insert(0) -= c;
//             *self.d.entry(x).or_insert(0) -= if self.d[&x] < c { self.d[&x] } else { c };
//             if self.d[&x] <= 0 {
//                 self.s.remove(&x);
//                 // *self.d.entry(x).or_insert(0) = 0;
//             }
//         }
//     }

//     fn get_diff(&self) -> u64 {
//         self.s.iter().next_back().unwrap() - self.s.iter().next().unwrap()
//     }
// }
