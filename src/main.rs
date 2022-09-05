use raylib::prelude::*;
mod ray;

use ray::Ray;

pub const WIDTH:i32 = 800;
pub const HEIGHT:i32 = 600;

fn main() {
    
    let (mut rl, thread) = raylib::init()
    .size(WIDTH, HEIGHT)
    .title("Raycasting")
    .build(); 


    let mut r1 = Ray{p0: Vector2 { x: 100f32, y:100f32}, p1: Vector2 { x: 100f32, y: 200f32}, is_wall:false};
    let mut r2 = Ray{p0: Vector2 { x: 200f32, y:150f32}, p1: Vector2 { x: 200f32, y: 200f32}, is_wall:false};


    while !rl.window_should_close() {
        let mut mouse_pos = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);
        


        d.clear_background(Color::BLACK);
        r1.p0 = mouse_pos;

        d.draw_circle_v(r1.intersectSegment(&mut r2).1, 10f32, Color::WHITE);

        r1.render(&mut d);
        r2.render(&mut d);



    }

}
