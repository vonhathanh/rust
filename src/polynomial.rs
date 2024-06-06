use std::fmt;

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
