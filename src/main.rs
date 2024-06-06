pub mod polynomial;
use polynomial::Polynomial;

fn main() {
    let zero = Polynomial::new(vec![]);
    let f = Polynomial::new(vec![1.0, 2.0, 3.0]);
    let g = Polynomial::new(vec![-8.0, 17.0, 0.0, 5.0]);

    println!("zero poly: {}", zero);
    println!("f: {}", f);
    println!("g: {}", g);
}
