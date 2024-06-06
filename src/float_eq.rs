use std::hash::{Hash, Hasher};

pub struct FloatEq(f32);

impl FloatEq {
    pub fn new(value: f32) -> Self {
        FloatEq(value)
    }
}

impl Eq for FloatEq {}

impl PartialEq for FloatEq {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < f32::EPSILON
    }
}

impl Hash for FloatEq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
