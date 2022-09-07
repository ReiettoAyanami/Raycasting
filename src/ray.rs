use raylib::prelude::*;
use raylib::math::*;
use std::cmp::min;
use std::cmp::max;
use std::collections::btree_set::Intersection;




pub struct Ray{

    pub p0:Vector2,
    pub p1: Vector2,
    pub is_obstacle:bool,
    pub length: f32,

    // default WHITE
    pub render_color: Color



    

}

impl Ray {
    

    pub fn new(p0:Vector2, p1:Vector2, is_obstacle:bool, render_color: Color) -> Ray{

        let length = p0.distance_to(p1);
        Ray { p0: p0, p1: p1, is_obstacle: is_obstacle, length: length, render_color: render_color}

    }


    pub fn new_from_angle(position:Vector2, length:f32, a:f32, is_obstacle:bool, render_color: Color) -> Ray{

        let p0 = position + Vector2::new(a.cos() * length, -a.sin() * length);

        Ray{p0: p0, p1: position, length: length, is_obstacle:is_obstacle, render_color: render_color}

    }


    pub fn set_angle(&mut self, a:f32){

        self.p0 = self.p1 + Vector2::new(a.cos() * self.length, -a.sin() * self.length);

    }

    pub fn point_to(&mut self, position:Vector2) -> f32{
        let delta = position - self.p1;

        let m:f32 = -delta.y / delta.x;
        let mut a:f32 = m.atan();

        if delta.x < 0.0{

            a += std::f32::consts::PI;



        } else if delta.x == 0.0{

            a = -std::f32::consts::FRAC_PI_2 * delta.y.signum();


        }   


        self.p0 = self.p1 + Vector2::new(a.cos() * self.length, -a.sin() * self.length);


        a
    }


    pub fn intersect_line(&mut self, r: &mut Ray) -> (bool, Vector2){

        if !r.is_obstacle{ return (false, Vector2::new(f32::NAN, f32::NAN))}

        let mut is_colliding = false;

        let a1 = self.p1.y - self.p0.y;
        let b1 = self.p0.x - self.p1.x;
        let c1 = a1 * self.p0.x + b1 * self.p0.y;

        let a2 = r.p1.y - r.p0.y;
        let b2 = r.p0.x - r.p1.x;
        let c2 = a2 * r.p0.x + b2 * r.p0.y;


        let den = a1 * b2 - a2 * b1;

        if den != 0.0{

            is_colliding = true;

        }



        return (is_colliding, Vector2{x: (b2 * c1 - b1 * c2) / den, y: (a1 * c2 - a2 * c1) / den });

    }

    pub fn intersect_segment(&mut self, r: &mut Ray) -> (bool, Vector2){

        if !r.is_obstacle{ return (false, Vector2::new(f32::NAN, f32::NAN))}


        let intersection = self.intersect_line(r).1;

        let r0 = Vector2::new((intersection.x - self.p0.x) / (self.p1.x - self.p0.x), (intersection.y - self.p0.y) / (self.p1.y - self.p0.y));
        let r1 = Vector2::new((intersection.x - r.p0.x) / (r.p1.x - r.p0.x), (intersection.y - r.p0.y) / (r.p1.y - r.p0.y));



        if ((r0.x > 0.0 && r0.x < 1.0) || (r0.y > 0.0 && r0.y < 1.0)) && ((r1.x > 0.0 && r1.x < 1.0) || (r1.y > 0.0 && r1.y < 1.0)){

            return  (true, intersection);

        }else {

            return (false, Vector2::new(std::f32::NAN, std::f32::NAN));
        }

    }


    pub fn update_length(&mut self, r: &mut Ray){

        let intersection = self.intersect_segment(r);


        if self.p0.distance_to(intersection.1) < self.length && intersection.0 {
            self.p0 = intersection.1;
        }

    }

    // ( NOTE )
    // Sta roba è da provare anche perché ho dubbi del suo funzionamento.

    // pub fn find_closest_ray<'a>(&mut self, rays: &'a mut Vec<&mut Ray>) -> (Vector2,&'a mut Ray){

    //     let mut closest_ray_index:usize = 0;
    //     let mut closest_intersection: Vector2 = Vector2 { x: f32::NAN, y: f32::NAN };
    //     let mut least_distance: f32 = f32::INFINITY;

    //     for i in 0..rays.len(){

    //         let intersection = self.intersect_segment(rays[i]);
    //         let d = self.p0.distance_to(intersection.1);
    //         if d < least_distance{

    //             least_distance = d;
    //             closest_intersection = intersection.1;
    //             closest_ray_index = i;


    //         }

    //     }



    //     (closest_intersection, rays[closest_ray_index])


    // }


    pub fn render(&self, d: &mut RaylibDrawHandle){


        d.draw_line_v(self.p0, self.p1, self.render_color);
    }


}

