use raylib::prelude::*;

pub fn get_angle_between(p0:Vector2,p1:Vector2) -> f32{

    let delta = p0 - p1;

    let m:f32 = -delta.y / delta.x;
    let mut a:f32 = m.atan();

    if delta.x < 0.0{

        a += std::f32::consts::PI;



    } else if delta.x == 0.0{

        a = -std::f32::consts::FRAC_PI_2 * delta.y.signum();


    }   

    a

}