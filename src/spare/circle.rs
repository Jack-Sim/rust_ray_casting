pub struct Circle {
    pub pos_x: f64,
    pub pos_y: f64,
    pub radius: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    canv_width: u32,
    canv_height: u32,
}

impl Circle {
    pub fn new(pos_x: f64, pos_y: f64, radius: f64, vel_x: f64, vel_y: f64, canv_width: u32, canv_height: u32) -> Result<Circle, &'static str> {
        if radius >= (canv_width / 2) as f64 || radius >= (canv_height / 2 )as f64 {
            return Err("Your circle is off the page");
        } else {
            return Ok(Circle {pos_x, pos_y, radius, vel_x, vel_y, canv_width, canv_height});
        }
    }

    pub fn update(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.check_edge();
    }
    
    pub fn check_edge(&mut self) {
        if self.pos_x < 0.0 || self.pos_x > self.canv_width as f64 - self.radius {
            self.vel_x *= -1.0;
        }
        if self.pos_y < 0.0 || self.pos_y > self.canv_height as f64 - self.radius {
            self.vel_y *= -1.0;
        }
    }
}