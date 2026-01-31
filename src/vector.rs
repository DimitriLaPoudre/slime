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
    T: Copy + Into<f32>,
{
    pub fn distance(&self, other: Vector2D<T>) -> f32 {
        let dir = Vector2D {
            x: other.x.into() - self.x.into(),
            y: other.y.into() - self.y.into(),
        };

        let dist_sq: f32 = dir.x * dir.x + dir.y * dir.y;
        dist_sq.sqrt()
    }
}
