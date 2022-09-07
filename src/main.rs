
mod ray;
mod raycaster;
mod utils;

use utils::*;
use ray::Ray;
use raycaster::Raycaster;
use raylib::prelude::*;
pub const WIDTH:i32 = 800;
pub const HEIGHT:i32 = 600;

fn main() {
    
    let (mut rl, thread) = raylib::init()
    .size(WIDTH, HEIGHT)
    .title("Raycasting")
    .msaa_4x()
    .build(); 



    //let mut r = Ray::new_from_angle(Vector2::new(100f32, 100f32), 50f32,0.0, false, Color::BLUE);

    let mut raycaster: Raycaster = Raycaster::new(Vector2::new(100f32,100f32), std::f32::consts::FRAC_PI_6, 360f32.to_radians(), 1.0/36.0f32, WIDTH as f32, Color::WHITE);


    let mut obstacles: Vec<Ray> = vec![];

    for i in 0..50{
        
        let (rx1, ry1): (i32, i32) = (get_random_value(0i32, WIDTH), get_random_value(0, HEIGHT));
        let angle: i32 = get_random_value(0, 359);
        let length: i32 = get_random_value(25, 100);

        let p1 = Vector2{x: rx1 as f32, y: ry1 as f32};

        let r:Ray = Ray::new_from_angle(p1, length as f32, angle as f32, true, Color::RED);

        

        obstacles.push(r);

    }



    while !rl.window_should_close() {
        let mut mouse_pos = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);




        //r.point_to(mouse_pos);
        
        d.clear_background(Color::BLACK);
        
        raycaster.follow(mouse_pos, 20.0);

        raycaster.point_to(mouse_pos);

        for obstacle in &mut obstacles{
            
            raycaster.update(obstacle);
            obstacle.render(&mut d);


        }


        //println!("{}", d.get_fps());

        raycaster.render(&mut d);




    }

}
