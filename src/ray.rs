use crate::boundary::Boundary;
use std::marker::Copy;
use std::clone::Clone;

#[derive(Copy, Clone)]
pub struct Ray {
    pub pos_x: f64,
    pub pos_y: f64,
    pub direction: [f64;2],
}

impl Ray {
    pub fn new(pos_x: f64,pos_y: f64, direction: [f64;2]) -> Ray {
        return Ray {pos_x, pos_y, direction}
    }

    pub fn cast(&self, wall: &Boundary) -> [f64; 2] {
        let x1 = wall.start[0];
        let y1 = wall.start[1];
        let x2 = wall.end[0];
        let y2 = wall.end[1];
        let x3 = self.pos_x;
        let y3 = self.pos_y;
        let x4 = self.pos_x + self.direction[0];
        let y4 = self.pos_y + self.direction[1];

        let denom = (x1 - x2)*(y3-y4) - (x3 - x4) * (y1 - y2);
        if denom == 0.0 {
            return [0.0,0.0];
        } else {
            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
            let u = -1.0 * ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;
            
            if t > 0.0 && t < 1.0 && u > 0.0 {
                let pt: [f64;2] = [x1 + t *(x2 - x1), y1 + t * (y2 - y1)];
                return pt;
            } else {
                return [0.0, 0.0];
            }
        }
    }
}
