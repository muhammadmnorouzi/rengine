use std::convert::From;
use std::f32::consts::PI;

pub struct Degree {
    value: f32
}

impl Degree {
    pub fn from_radians(rad : f32) -> Self {
        (rad * 180f32 / PI).into()
    }

    pub fn as_degrees(&self) -> f32 {
        self.value
    }

    pub fn as_radians(&self) -> f32 {
        self.value * PI / 180f32
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