use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2D { x, y }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Add<Output = T>,
{
    pub fn add(&self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn vadd(&self, value: T) -> Vector2D<T> {
        Vector2D {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn sub(&self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn vsub(&self, value: T) -> Vector2D<T> {
        Vector2D {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Mul<Output = T>,
{
    pub fn mul(&self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn vmul(&self, value: T) -> Vector2D<T> {
        Vector2D {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Div<Output = T>,
{
    pub fn div(&self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    pub fn vdiv(&self, value: T) -> Vector2D<T> {
        Vector2D {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn delta(&self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Into<f32> + Sub<Output = T>,
{
    pub fn length(&self) -> f32 {
        let dist_sq: f32 = self.x.into() * self.x.into() + self.y.into() * self.y.into();
        dist_sq.sqrt()
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Into<f32> + Sub<Output = T> + Div<Output = T>,
{
    pub fn normalize(&self) -> Vector2D<f32> {
        let length = self.length();
        Vector2D {
            x: self.x.into() / length,
            y: self.y.into() / length,
        }
    }
}
