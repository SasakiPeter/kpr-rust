use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    println!("{}", a * d - b * c);

    // input!(a: (i32, i32, i32, i32));
    // println!("{}", a.0 * a.3 - a.1 * a.2);

    // input! {
    //     a: [i32; 4]
    // }
    // input!(a: [i32; 4]);
    // println!("{}", a[0] * a[3] - a[1] * a[2]);
}

// fn type_of<T>(_: T) -> String {
//     std::any::type_name::<T>().to_string()
// }
