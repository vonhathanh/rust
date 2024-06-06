pub mod float_eq;
pub mod polynomial;

use polynomial::{interpolate, Polynomial};

fn main() {
    let zero = Polynomial::new(vec![]);
    let f = Polynomial::new(vec![1.0, 2.0, 3.0]);
    let g = Polynomial::new(vec![-8.0, 17.0, 0.0, 5.0]);

    println!("zero poly: {}", zero);
    println!("f: {}", f);
    println!("g: {}", g);
    let sum_gf = f + g;
    println!("sum_gf: {}", sum_gf);

    let points1 = vec![(1.0, 1.0)];
    println!("{}", interpolate(&points1).unwrap());

    let points2 = vec![(1.0, 1.0), (2.0, 0.0)];
    println!("{}", interpolate(&points2).unwrap());

    let points3 = vec![(1.0, 1.0), (2.0, 4.0), (7.0, 9.0)];
    println!("{}", interpolate(&points3).unwrap());
    let f = interpolate(&points3).unwrap();
    println!("{}, {}, {}", f.evaluate(1.0), f.evaluate(2.0), f.evaluate(7.0))
}
