use crate::float_eq::FloatEq;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Mul};
use std::{fmt, vec};

pub struct Polynomial {
    pub coefficients: Vec<f32>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f32>) -> Self {
        Polynomial { coefficients }
    }

    pub fn evaluate(&self, x: f32) -> f32 {
        let mut sum = 0.0;
        for (i, coeff) in self.coefficients.iter().enumerate() {
            sum += coeff * x.powi(i.try_into().unwrap());
        }
        sum
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, value) in self.coefficients.iter().enumerate() {
            if *value == 0.0 {
                continue;
            }
            if i == 0 {
                write!(f, "{}", value)?;
            } else if i == 1 {
                write!(f, " + {}x", value)?;
            } else {
                write!(f, " + {}x^{}", value, i)?;
            }
        }
        Ok(())
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let left_length = self.coefficients.len();
        let right_length = other.coefficients.len();
        let max_length = left_length.max(right_length);

        let mut res: Vec<_> = vec![0.0; max_length];

        for i in 0..max_length {
            if i < left_length {
                res[i] += self.coefficients[i];
            }
            if i < right_length {
                res[i] += other.coefficients[i];
            }
        }

        Polynomial { coefficients: res }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut coeff: Vec<_> = vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

        for i in 0..self.coefficients.len() {
            for j in 0..rhs.coefficients.len() {
                coeff[i + j] += self.coefficients[i] * rhs.coefficients[j];
            }
        }

        Polynomial { coefficients: coeff }
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        let right_length = rhs.coefficients.len();
        let max_length = right_length.max(self.coefficients.len());
        if self.coefficients.len() < max_length {
            self.coefficients.resize(max_length, 0.0);
        }
        for i in 0..right_length {
            self.coefficients[i] += rhs.coefficients[i];
        }
    }
}

fn single_term(points: &Vec<(f32, f32)>, i: usize) -> Polynomial {
    let mut term = Polynomial::new(vec![1.0]);

    let (xi, yi) = points[i];

    for (j, p) in points.iter().enumerate() {
        if j == i {
            continue;
        }
        let xj = p.0;
        term = term * Polynomial::new(vec![-xj / (xi - xj), 1.0 / (xi - xj)]);
    }
    term = term * Polynomial::new(vec![yi]);
    term
}

pub fn interpolate(points: &Vec<(f32, f32)>) -> Result<Polynomial, String> {
    if points.len() == 0 {
        return Err(String::from("Must provide at least one point"));
    }

    let mut x_points = vec![0.0; points.len()];
    for i in 0..points.len() {
        x_points[i] = points[i].0;
    }

    let set: HashSet<_> = x_points.iter().map(|x| FloatEq::new(*x)).collect();

    if set.len() < points.len() {
        return Err(String::from("All x values must unique"));
    }

    let mut terms = Polynomial::new(vec![]);

    for i in 0..points.len() {
        terms += single_term(points, i);
    }

    Ok(terms)
}
