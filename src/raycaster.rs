use crate::{ray::{Ray, self, NULL_RAY}, main};
use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;
use crate::utils::*;
use std::f32::consts::PI;



#[derive(Debug, Clone)]
pub struct Raycaster{

    pub position: Vector2,
    pub ray_length: f32,
    pub render_color: Color,
    pub rays: Vec<ray::Ray>,
    hitbox_size: f32,
    starting_angle: f32,
    velocity: Vector2,
    collisions: Vec<ray::Ray>,
    pub fov:f32
}


impl Raycaster{

    pub fn new(pos:Vector2, starting_angle: f32, fov: f32,n_rays:i32, ray_length:f32, render_color: Color) -> Raycaster{
        
        let mut rays:Vec<Ray> = vec![];
        let mut collisions:Vec<Ray> = vec![];
        let focal_length:f32 = 1f32 / (fov/2f32).tan();
        let mut x:f32 = 0f32;
        let mut t_angle:f32;
        let angle:f32 = starting_angle - (fov / 2f32);


        for i in 0..n_rays{

            x = 0.5f32 - (i as f32 / n_rays as f32);
            t_angle = f32::atan2(x, focal_length);

            let r:ray::Ray = ray::Ray::new_from_angle(pos, ray_length, angle + t_angle, false, render_color);
            rays.push(r);
            collisions.push(ray::NULL_RAY);
        }


        Raycaster{position: pos, ray_length: ray_length, render_color: render_color, rays:rays,hitbox_size: 10f32, starting_angle: starting_angle, velocity: Vector2 { x: 0.0, y: 0.0 }, collisions:collisions, fov:fov}

    }


    pub fn constrain(&mut self, constraints: Rectangle){

        
        if !constraints.check_collision_point_rec(self.position){

            self.position -= self.velocity;

        }


    }


    pub fn point_to(&mut self, position:Vector2){

        let angle:f32 = get_angle_between(self.position, position);
        let focal_length = (1f32 / (self.fov / 2f32).tan()) / 2f32; 
        let mut x = 0f32;
        let mut t_angle:f32;

        for i in 0..self.rays.len(){

            x = 0.5 - (i as f32/self.rays.len() as f32);
            t_angle = f32::atan2(x, focal_length);
            self.rays[i].set_angle(angle + t_angle + PI);


        }



    }   

 


        // MSN Account secondo Massimo Sandretti:
        // pub fn giorgia(Rc<RefCell<Mutex<Sus>>>)

    pub fn follow(&mut self, position:Vector2, offset:f32, dt:f32){

        let mut d = self.position.distance_to(position);

        d = f32::max(d - offset, 0.0);

        d /= 1.0;

        
        
        let a:f32 = get_angle_between(self.position, position);


        self.velocity = Vector2::new(d * -a.cos(), d * a.sin()) * dt;

        self.position = self.position + self.velocity;


        for ray in &mut self.rays{

            ray.p1 = self.position;

        }
    }

    pub fn get_rays_intersection_distance(&mut self) -> Vec<f32>{

        let mut d: Vec<f32> = vec![];

        for i in 0..self.rays.len(){

            d.push(self.rays[i].p0.distance_to(self.rays[i].p1));

        }

        d

    }

    pub fn get_colliding(&mut self) -> Vec<ray::Ray>{

        self.collisions.clone()
    
    }




    pub fn update(&mut self, r: &mut ray::Ray){



        for i in 0..self.rays.len(){

        
            self.collisions[i] = self.rays[i].intersect_segment(r).2;

            self.rays[i].update_length(r);
            
            

        }


        


    }


    pub fn render(&self, d: &mut RaylibDrawHandle){

        if d.is_key_pressed(KEY_G){

            for i in 0..self.collisions.len(){

                println!("{:?}", self.collisions[i]);

            }

        }

        for ray in &self.rays{
            ray.render(d);
        }


        for i in 0..self.rays.len() - 1 {

            d.draw_line_v(self.rays[i].p0, self.rays[i + 1].p0, self.render_color);


        }
    }


    pub fn render_fov(&self, d:&mut RaylibDrawHandle){

        self.rays[0].render(d);
        self.rays[self.rays.len() - 1].render(d);

        for i in 0..self.rays.len() - 1 {


            d.draw_line_v(self.rays[i].p0, self.rays[i + 1].p0, self.render_color);
            

        }


    }



}