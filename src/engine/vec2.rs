use std::convert::From;
use std::default::Default;
use std::ops::{Sub , Mul , AddAssign , MulAssign , SubAssign};
use crate::utilities::degree::Degree;

pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn get_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x.pow(2) + self.y.pow(2)) as f32)
    }

    pub fn normalized(&self) -> Self {
        let magnitude: f32 = self.magnitude();
        let (x , y) = self.get_xy();

        Self {
            x: (x as f32 / magnitude) as i32,
            y: (y as f32 / magnitude) as i32,
        }
    }

    pub fn distance(&self , other: &Self) -> f32 {
        let diff = self - other;
        diff.magnitude()
    }

    pub fn x_tendency(&self) -> f32 {
        f32::atan2(self.y as f32 , self.x as f32)
    }

    pub fn rotate(&self, d :Degree) -> Self {
        let (x , y) = self.get_xy();
        let (x , y) = (x as f32 , y as f32);
        let d = d.value();
        
        let xr = x * d.cos() - y * d.sin();
        let yr = y * d.cos() + x * d.sin();

        Self {
            x: xr as i32,
            y: yr as i32
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(xy: (i32, i32)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<i32> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: i32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<i32> for Vec2 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}


