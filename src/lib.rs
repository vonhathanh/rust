pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod float_eq;
pub mod polynomial;

use polynomial::{interpolate, Polynomial};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomial() {
        let zero = Polynomial::new(vec![]);
        let f = Polynomial::new(vec![1.0, 2.0, 3.0]);
        let g = Polynomial::new(vec![-8.0, 17.0, 0.0, 5.0]);

        assert_eq!(zero.to_string(), "");
        assert_eq!(f.to_string(), "1 + 2x + 3x^2");
        assert_eq!(g.to_string(), "-8 + 17x + 5x^3");

        let sum_gf = f + g;
        assert_eq!(sum_gf.to_string(), "-7 + 19x + 3x^2 + 5x^3");

        let points1 = vec![(1.0, 1.0)];
        assert_eq!(interpolate(&points1).unwrap().to_string(), "1");

        let points2 = vec![(1.0, 1.0), (2.0, 0.0)];
        assert_eq!(interpolate(&points2).unwrap().to_string(), "2 + -1x");

        let points3 = vec![(1.0, 1.0), (2.0, 4.0), (7.0, 9.0)];
        assert_eq!(interpolate(&points3).unwrap().to_string(), "-2.6666665 + 4x + -0.3333333x^2");

        let f = interpolate(&points3).unwrap();
        assert_eq!(f.evaluate(1.0), 1.0000002);
        assert_eq!(f.evaluate(2.0), 4.0);
        assert_eq!(f.evaluate(7.0), 9.000002);
    }
}