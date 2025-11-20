use std::fmt::Display;

pub struct Point<T,M> {
    pub x: T,
    pub y: M,
}

impl Point<f32, f32> {
    pub fn distance(&self) -> f32 {
        ((self.x.powi(2)) + (self.y.powi(2))).sqrt()
    }
}

impl<T,M> Point<T,M>
where 
    T: Display + PartialOrd<M>,
    M: Display + PartialOrd<T>,
{
    pub fn cmp(&self) {
        if self.x >= self.y {
            println!("Largest number is {}", self.x);
        } else {
            println!("Largest number is {}", self.y);
        }
    }
}