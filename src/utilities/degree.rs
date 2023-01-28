use std::convert::From;

pub struct Degree {
    value: f32
}

impl Degree {
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl Default for Degree {
    fn default() -> Self {
        Self { value: 0f32 }
    }
}

impl From<f32> for Degree {
    fn from(value: f32) -> Self {
        Self { value}
    }
}