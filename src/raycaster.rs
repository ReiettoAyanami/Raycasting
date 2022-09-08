use crate::{ray::{Ray, self, NULL_RAY}, main};
use raylib::prelude::*;
use crate::utils::*;



#[derive(Debug, Clone)]
pub struct Raycaster{

    pub position: Vector2,
    pub ray_length: f32,
    pub render_color: Color,
    pub rays: Vec<ray::Ray>,
    hitbox_size: f32,
    pub increment: f32, 
    starting_angle: f32,
    velocity: Vector2,
    collisions: Vec<ray::Ray>

}


impl Raycaster{

    pub fn new(pos:Vector2, starting_angle: f32, fov: f32, increment:f32, ray_length:f32, render_color: Color) -> Raycaster{
        
        let mut rays:Vec<ray::Ray> = vec![];
        let mut collisions:Vec<ray::Ray> = vec![];

        let n_iter: i32 = (fov / increment) as i32;

        for i in 0..n_iter{
            
            let a: f32 = starting_angle - (increment * i as f32);

            //let c = Color::color_from_hsv((increment * i as f32).to_degrees(), 1.0, 1.0);

            let temp_ray: ray::Ray = ray::Ray::new_from_angle(pos, ray_length, a, false, render_color);

            
            rays.push(temp_ray);
            collisions.push(NULL_RAY);

        }

        Raycaster{position: pos, ray_length: ray_length, render_color: render_color, rays:rays,hitbox_size: 10f32, starting_angle: starting_angle, increment:increment, velocity: Vector2 { x: 0.0, y: 0.0 }, collisions}

    }


    pub fn constrain(&mut self, constraints: Rectangle){

        
        if !constraints.check_collision_point_rec(self.position){

            self.position -= self.velocity;

        }


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