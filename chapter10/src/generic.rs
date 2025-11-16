pub struct Point<T,M> {
    pub x: T,
    pub y: M,
}

impl Point<f32, f32> {
    pub fn distance(&self) -> f32 {
        ((self.x.powi(2)) + (self.y.powi(2))).sqrt()
    }
}
