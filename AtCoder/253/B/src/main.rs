use proconio::input;

fn main() {
    input! {
        h: i32,
        _: i32,
        s: [String; h]
    }
    // println!("{:?}", S);

    // let mut a = Vec::new();
    let (mut a, mut b) = (0, 0);
    for (i, row) in s.iter().enumerate() {
        for (j, x) in row.as_str().chars().enumerate() {
            if x == 'o' {
                // let (a, b) = (i, j);
                // a.push(vec![i, j])
                // println!("({}, {})", i, j)
                a = (a - i as i32).abs();
                b = (b - j as i32).abs();
            }
        }
    }
    println!("{:?}", a + b)

    // println!("{}", type_of(S));
}

// fn type_of<T>(_: T) -> String {
//     std::any::type_name::<T>().to_string()
// }
