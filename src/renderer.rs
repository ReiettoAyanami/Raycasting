use raylib::prelude::*;
use crate::Ray;
use crate::Raycaster;


pub struct Renderer{

    render_rectangle: Rectangle,
    scans: i32,
    
    


}

impl Renderer{

    pub fn new(render_rectangle: Rectangle, scans: i32) -> Renderer{

        Renderer {render_rectangle:render_rectangle, scans: scans}

    }


    fn render_rectangle_at_scan(&self,scan: i32 ,height:i32,d: &mut RaylibDrawHandle, color: Color){

        let scan_width: f32 = self.render_rectangle.width / self.scans as f32;    
        
        d.draw_rectangle(scan_width as i32 * scan + self.render_rectangle.x as i32, self.render_rectangle.y as i32 + self.render_rectangle.height as i32 / 2 - height as i32/2, scan_width as i32, height as i32, color);

    }


    pub fn render(&mut self, d: &mut RaylibDrawHandle, raycaster: &mut Raycaster){
        let increment = raycaster.increment;
        let fov = increment * self.scans as f32;
        let rays = raycaster.get_rays_intersection_distance();



        for i in 0..self.scans{
            let fixed_distance = rays[i as usize] * ((fov / 2.0) - (increment * i as f32)).cos();
            let h = self.render_rectangle.height - fixed_distance;
            
            

            let c = Color::color_from_normalized(Vector4::new((1.0/(rays[i as usize] * 0.01 )).min(1.0), (1.0/(rays[i as usize]* 0.01 )).min(1.0), (1.0/(rays[i as usize] * 0.01 )).min(1.0), 1.0));

            self.render_rectangle_at_scan(i, h as i32, d, c);

        }

    }

}