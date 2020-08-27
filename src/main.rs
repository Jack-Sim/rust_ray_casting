extern crate piston_window;
extern crate perlin_noise as perlin;

use piston_window::*;
use rand::Rng;

mod boundary;
mod ray;
mod particle;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 640;
pub const LINE_COL: [f32;4] = [1.0, 1.0, 1.0, 1.0];
pub const LINE_SIZE: f64 = 1.0;
pub const RAY_COUNT: usize = 720;

fn main() {
    // Create the particle struct with a start position x = 100, y = 300;
    let mut my_particle = particle::Particle::new(100.0, 300.0);
    
    // Create 5 new boundaries with random positions on the window;
    let mut my_boundaries: [boundary::Boundary; 5] = [boundary::Boundary::new(0.0, 0.0, 0.0, 0.0); 5];
    let mut rng = rand::thread_rng();
    for n in 1..5 {
        let x1: f64 = rng.gen_range(0, WIDTH) as f64;
        let y1: f64 = rng.gen_range(0, HEIGHT) as f64;
        let x2: f64 = rng.gen_range(0, WIDTH) as f64;
        let y2: f64 = rng.gen_range(0, HEIGHT) as f64;
        my_boundaries[n] = boundary::Boundary::new(x1,y1,x2,y2);
    }

    // Create the PistonWindow to display the animation;
    // Window title = "Ray Casting", Width and Height equal to constants
    let mut window: PistonWindow = 
            WindowSettings::new("Ray Casting", [WIDTH, HEIGHT])
            .exit_on_esc(true).build().unwrap();
    // The animation loop for the window
    while let Some(e) = window.next(){
        window.draw_2d(&e, |c, g, _device| {

            clear([0.0, 0.0, 0.0, 1.0], g);
            let mut cast_rays: [[f64;2];RAY_COUNT] = [[0.0, 0.0];RAY_COUNT];
            for i in 0..RAY_COUNT {
                for n in 0..5 {
                    let cast = my_particle.rays[i].cast(&my_boundaries[n]);
                    if cast != [0.0,0.0] {
                        if cast_rays[i] == [0.0, 0.0] {
                            cast_rays[i] = cast;
                        } else if cast_rays[i] != [0.0, 0.0] {
                            let dist1 = ((my_particle.pos_x - cast[0]).powf(2.0) + (my_particle.pos_y - cast[1]).powf(2.0)).sqrt();
                            let dist2 = ((my_particle.pos_x - cast_rays[i][0]).powf(2.0) + (my_particle.pos_y - cast_rays[i][1]).powf(2.0)).sqrt();
                            if dist1 < dist2 {
                                cast_rays[i] = cast;
                            }
                        }
                    }
                    
                }
                if cast_rays[i] != [0.0, 0.0] {
                    line([1.0, 1.0, 1.0, 0.1], 0.5, [my_particle.rays[i].pos_x, my_particle.rays[i].pos_y, cast_rays[i][0], cast_rays[i][1]], c.transform, g);
                }
            }
            for n in 0..5{
                line(LINE_COL, LINE_SIZE, my_boundaries[n].display(), c.transform, g);
            }
            my_particle.update(WIDTH, HEIGHT); 
        });   
    }
}

