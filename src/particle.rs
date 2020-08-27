use crate::ray::Ray;
use rand::Rng;

pub struct Particle {
    pub pos_x: f64,
    pub pos_y: f64,
    max_velo_x: f64,
    max_velo_y: f64,
    pub rays: [Ray; 720]
}

impl Particle {
    pub fn new (pos_x: f64, pos_y: f64) -> Particle {
        let mut rays: [Ray; 720] = [Ray::new(pos_x, pos_y, [0.0, 0.0]); 720];
        for i in 1..720 {
            let degrees = i as f64 / 2.0;
            let rads = degrees.to_radians();
            rays[i].direction = [rads.cos(), rads.sin()];
        }

        return Particle {pos_x, pos_y, rays, max_velo_x: 2.0, max_velo_y: 2.0};
    }

    pub fn update(&mut self, width: u32, height: u32) {
        let mut rng = rand::thread_rng();
        if self.max_velo_x > 0.0 {
            self.pos_x = self.pos_x + rng.gen_range(0.0, self.max_velo_x);
        } else {
            self.pos_x = self.pos_x + rng.gen_range(self.max_velo_x, 0.0);
        }
        if self.max_velo_y > 0.0 {
            self.pos_y = self.pos_y + rng.gen_range(0.0, self.max_velo_y);
        } else {
            self.pos_y = self.pos_y + rng.gen_range(self.max_velo_y, 0.0);
        }
        for i in 1..720 {
            self.rays[i].pos_x = self.pos_x;
            self.rays[i].pos_y = self.pos_y;
        }
        if self.pos_x > width as f64 {
            self.max_velo_x *= -1.0;
            self.pos_x = width as f64;
        }
        if self.pos_x < 0.0 {
            self.max_velo_x *= -1.0;
            self.pos_x = 0.0;
        }
        if self.pos_y > height as f64 { 
            self.max_velo_y *= -1.0;
            self.pos_y = height as f64;
        }
        if self.pos_y < 0.0 {
            self.max_velo_y *= -1.0;
            self.pos_y = 0.0;
        }
    }
}