use proconio::input;

fn main() {
    input! {
        h: i32,
        _w: i32,
        s: [String; h]
    }
    // println!("{:?}", S);

    let mut pos = vec![];
    for (i, row) in s.iter().enumerate() {
        // // &String→char
        // println!("{:?}", row.as_str().chars());

        for (j, x) in row.as_str().chars().enumerate() {
            if x == 'o' {
                pos.push((i as i32, j as i32));
            }
        }
    }

    let (a, b) = (pos[0], pos[1]);
    println!("{}", (a.0 - b.0).abs() + (a.1 - b.1).abs())

    // let (mut a, mut b) = (0, 0);
    // for (i, row) in s.iter().enumerate() {
    //     for (j, x) in row.as_str().chars().enumerate() {
    //         if x == 'o' {
    //             // let (a, b) = (i, j);
    //             // a.push(vec![i, j])
    //             // println!("({}, {})", i, j)
    //             a = (a - i as i32).abs();
    //             b = (b - j as i32).abs();
    //         }
    //     }
    // }
    // println!("{:?}", a + b)

    // println!("{}", type_of(S));
}

// fn type_of<T>(_: T) -> String {
//     std::any::type_name::<T>().to_string()
// }
