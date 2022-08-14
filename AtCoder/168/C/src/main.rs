use proconio::input;
// fn main() {
//     input!(a: f64, b: f64, h: f64, m: f64);
//     let theta: f64 = ((60. * h + m) / 2. - 6. * m).abs();
//     // println!("{}, {}", (60. * h + m) / 2., 6. * m);
//     let c2: f64 = a.powf(2.) + b.powf(2.) - 2. * a * b * theta.to_radians().cos();
//     println!("{}", c2.sqrt());
// }

fn main() {
    input!(a: f64, b: f64, h: f64, m: f64);

    // let (deg_h, deg_m) = ((60. * h + m) / 2., 6. * m);
    // let v1: V = V {
    //     x: a * deg_h.to_radians().cos(),
    //     y: a * deg_h.to_radians().sin(),
    // };
    // let v2: V = V {
    //     x: b * deg_m.to_radians().cos(),
    //     y: b * deg_m.to_radians().sin(),
    // };

    let (rad_h, rad_m) = (((60. * h + m) / 2.).to_radians(), (6. * m).to_radians());
    let v1 = V::polar_to_cartesian(a, rad_h);
    let v2 = V::polar_to_cartesian(b, rad_m);
    println!("{}", v1.sub(v2).length());
}
struct V {
    x: f64,
    y: f64,
}

trait Vector {
    fn sub(&self, q: V) -> V;
    fn length(&self) -> f64;
    fn polar_to_cartesian(a: f64, theta: f64) -> V;
}

impl Vector for V {
    fn sub(&self, q: V) -> V {
        V {
            x: self.x - q.x,
            y: self.y - q.y,
        }
    }

    fn length(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }

    fn polar_to_cartesian(a: f64, theta: f64) -> V {
        V {
            x: a * theta.cos(),
            y: a * theta.sin(),
        }
    }
}
