const MOD: i64 = 1_000_000_007;
fn main() {
    // 文字列リテラル
    println!("\"Hello\", #\\world!");

    // 生文字列リテラル
    println!(r#""Hello", world## \\"#);

    println!("{}", MOD);

    let hoge;
    // _はどこにいくつ挟んでも動くみたい
    hoge = 1_000_000_007_____i32;
    println!("{}", hoge);
    println!("{}", type_of(hoge));

    const AVOGADRO: f64 = 6.02e+23;
    println!("{}", AVOGADRO);
}

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
