use raylib::prelude::*;
use crate::utils::*;
use crate::{ray::{Ray, self}, main};


pub struct Raycaster{

    pub position: Vector2,
    pub ray_length: f32,
    pub render_color: Color,
    rays: Vec<Ray>,
    hitbox_size: f32,
    increment: f32, 
    starting_angle: f32

}


impl Raycaster{

    pub fn new(pos:Vector2, starting_angle: f32, fov: f32, increment:f32, ray_length:f32, render_color: Color) -> Raycaster{
        
        let mut rays:Vec<Ray> = vec![];
        let n_iter: i32 = (fov / increment) as i32;

        for i in 0..n_iter{
            
            let a: f32 = starting_angle - (increment * i as f32);

            let c = Color::color_from_hsv((increment * i as f32).to_degrees(), 1.0, 1.0);

            let temp_ray: Ray = Ray::new_from_angle(pos, ray_length, a, false, c);


            rays.push(temp_ray);


        }

        Raycaster{position: pos, ray_length: ray_length, render_color: render_color, rays:rays,hitbox_size: 10f32, starting_angle: starting_angle, increment:increment}

    }



    pub fn point_to(&mut self, position:Vector2){

        let fov = self.increment * (self.rays.len() -1) as f32;
        
        let mut relative_angle: f32 = fov / 2.0;

        for ray in &mut self.rays{

            let a:f32 = relative_angle + get_angle_between(position, ray.p1);

            relative_angle -= self.increment;

            ray.set_angle(a);

        }
        


    }   

 


        // MSN Account secondo Massimo Sandretti:
        // pub fn giorgia(Rc<RefCell<Mutex<Sus>>>)

    pub fn follow(&mut self, position:Vector2, offset:f32){

        let mut d = self.position.distance_to(position);

        d = f32::max(d - offset, 0.0);

        d /= 1000.0;

        let a:f32 = get_angle_between(self.position, position);

        self.position = self.position + Vector2::new(d * -a.cos(), d * a.sin());


        for ray in &mut self.rays{

            ray.p1 = self.position;

        }
    }


    pub fn update(&mut self, r: &mut Ray){



        for ray in &mut self.rays{




            ray.update_length(r);

        }


    }


    pub fn render(&self, d: &mut RaylibDrawHandle){


        for ray in &self.rays{
            ray.render(d);
        }

    }




}