fn main() {
    let length: i32;
    length = 100;

    println!("{}", length.pow(2));

    // 型注釈をつけよう
    let x: u32 = 50;
    println!("{}", type_of(x))
}

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
