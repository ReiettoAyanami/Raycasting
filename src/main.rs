use raylib::prelude::*;
mod ray;
mod raycaster;
mod utils;
mod renderer;

use utils::*;
use ray::Ray;

use renderer::Renderer;
use raycaster::Raycaster;               

pub const WIDTH:i32 = 800;
pub const HEIGHT:i32 = 600;
use std::clone::*;

fn main() {
    
    let (mut rl, thread) = raylib::init()
    .size(WIDTH * 2, HEIGHT)
    .title("Raycasting Rust")
    .msaa_4x()
    .build(); 


    let mut raycaster: Raycaster = Raycaster::new(Vector2::new(100f32,100f32), std::f32::consts::FRAC_PI_6,90f32.to_radians(), 800, WIDTH as f32, Color{r: 255, g: 255, b: 255, a: 255});
    
    let mut obstacles: Vec<Ray> = vec![];

    let mut renderer = Renderer::new(Rectangle::new(WIDTH as f32, 0f32, WIDTH as f32, HEIGHT as f32),raycaster.rays.len() as i32);

    for i in 0..10{
        
        let (rx1, ry1): (i32, i32) = (get_random_value(0i32, WIDTH), get_random_value(0, HEIGHT));
        let (rx0, ry0): (i32, i32) = (get_random_value(0i32, WIDTH), get_random_value(0, HEIGHT));


        let p1 = Vector2{x: rx1 as f32, y: ry1 as f32};
        let p0 = Vector2{x: rx0 as f32, y: ry0 as f32};

        let r:Ray = Ray::new(p0,p1,true,Color::RED);
        obstacles.push(r);

    }


    obstacles.push(Ray::new(Vector2::new(WIDTH as f32 + 1.0, 0f32), Vector2::new(WIDTH as f32 + 1.0, HEIGHT as f32), true, Color::BLUE));
    obstacles.push(Ray::new(Vector2::new(- 1.0, -1.0 ), Vector2::new(WIDTH as f32 + 1.0, -1.0), true, Color::BLUE));
    obstacles.push(Ray::new(Vector2::new(- 1.0, HEIGHT as f32), Vector2::new(WIDTH as f32 + 1.0, HEIGHT as f32+ 1.0), true, Color::BLUE));
    obstacles.push(Ray::new(Vector2::new(- 1.0, -1.0 ), Vector2::new(- 1.0, HEIGHT as f32 + 10 as f32), true, Color::BLUE));


    while !rl.window_should_close() {
        let mut mouse_pos = rl.get_mouse_position();
        let mut dt = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        


        //r.point_to(mouse_pos);
        
        d.clear_background(Color::BLACK);
        
        raycaster.follow(mouse_pos, 10.0, dt);

        raycaster.constrain(Rectangle::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32));

        raycaster.point_to(mouse_pos);

        
        

        for obstacle in &mut obstacles{
            
            raycaster.update(obstacle);
            obstacle.render(&mut d);


        }
        

        d.draw_text(&format!("{}", d.get_fps()), WIDTH / 2, 0, 20, Color::RAYWHITE);
        //println!("{:?}/n/n/n",raycaster.get_rays_intersection_distance());
        renderer.render(&mut d, &mut raycaster);
        raycaster.render(&mut d);


        



    }

}
