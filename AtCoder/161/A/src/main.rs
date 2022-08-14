use proconio::input;

fn main() {
    // input! {
    //     x: i32,
    //     y: i32,
    //     z: i32,
    // }
    // input!(x: i32, y: i32, z: i32);
    // println!("{} {} {}", z, x, y);

    // input! {
    //     mut a: [u32; 3]
    // }
    input!(mut a: [u32; 3]);
    a.swap(0, 1);
    a.swap(0, 2);
    println!("{} {} {}", a[0], a[1], a[2]);

    // println!("{:?}", a);
    // println!("{}", type_of(a));
}

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
