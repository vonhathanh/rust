use std::fmt;
use std::ops::Add;

pub struct Polynomial {
    pub coefficients: Vec<f32>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f32>) -> Self {
        Polynomial { coefficients }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, value) in self.coefficients.iter().enumerate() {
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
