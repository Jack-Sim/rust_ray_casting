use std::marker::Copy;
use std::clone::Clone;

#[derive(Copy, Clone)]
pub struct Boundary {
    pub start: [f64;2],
    pub end: [f64;2]
}

impl Boundary {
    pub fn new (x1: f64, y1: f64, x2: f64, y2: f64) -> Boundary {
        return Boundary {start: [x1, y1], end: [x2, y2]};
    }

    pub fn display(&self) -> [f64;4] {
        return [self.start[0], self.start[1],self.end[0],self.end[1]];
    }
}