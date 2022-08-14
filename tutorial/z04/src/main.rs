fn main() {
    println!("{}, {}, {}", 14 / 3, 14 % 3, type_of(2));

    println!("{}, {}", 7.5 / 2.5, type_of(7.5 / 2.5));

    // 5**2 はできなさそう
    println!("{}", 5_i32.pow(2));
}

// fn divmod(x i32, y i32)->(i32, i32){
//     return x/y, x%y
// }

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
