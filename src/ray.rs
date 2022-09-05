use raylib::prelude::*;
use raylib::math::*;
use std::cmp::min;
use std::cmp::max;
use std::collections::btree_set::Intersection;


pub struct Ray{

    pub p0:Vector2,
    pub p1: Vector2,
    pub is_wall:bool


}

impl Ray {
    
    pub fn intersectLine(&mut self, r: &mut Ray) -> (bool, Vector2){

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

    pub fn intersectSegment(&mut self, r: &mut Ray) -> (bool, Vector2){

        let intersection = self.intersectLine(r).1;

        let r0 = Vector2::new((intersection.x - self.p0.x) / (self.p1.x - self.p0.x), (intersection.y - self.p0.y) / (self.p1.y - self.p0.y));
        let r1 = Vector2::new((intersection.x - r.p0.x) / (r.p1.x - r.p0.x), (intersection.y - r.p0.y) / (r.p1.y - r.p0.y));



        if ((r0.x >= 0.0 && r0.x <= 1.0) || (r0.y >= 0.0 && r0.y <= 1.0)) && ((r1.x >= 0.0 && r1.x <= 1.0) || (r1.y >= 0.0 && r1.y <= 1.0)){

            return  (true, intersection);

        }else {
            return (false, Vector2::new(std::f32::NAN, std::f32::NAN));
        }

    }


    pub fn render(&self, d: &mut RaylibDrawHandle){


        d.draw_line_v(self.p0, self.p1, Color::WHITE);


    }


}

