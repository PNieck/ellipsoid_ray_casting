extern crate nalgebra as na;

use na::{Point2, Point3, UnitVector3, Vector2, Vector3};
use objects::{
    Camera,
    Canvas,
    Ellipse,
    Color,
    ellipse::HitRecord,
    CAMERA_CENTER
};
use winit::window::Window;

mod math;
mod objects;


pub struct Scene {
    camera: Camera,
    pub ellipse: Ellipse,
    pub canvas: Canvas,
    
    brightness: f32,

    cur_block_size: u32,
    max_block_size: u32,
}


impl Scene {
    pub fn new(window: &Window) -> Scene {
        Scene {
            camera: Camera::new(5.0, 5.0),
            ellipse: Ellipse::new(
                1.0, 2.0, 3.0,
                &Point3::new(0.0_f32, 0.0, 0.0),
                Color::from_rgb(239, 245, 66),
            ),
            canvas: Canvas::new(window),
            brightness: 2.0,
            cur_block_size: 81,
            max_block_size: 81
        }
    }

    pub fn render(&self) {
        self.canvas.render();
    }

    pub fn update(&mut self) {
        let miss_color: Color = Color::from_rgb(120, 120, 120);

        let points_x = u32::div_ceil(self.canvas.get_width(), self.cur_block_size);
        let points_y = u32::div_ceil(self.canvas.get_height(), self.cur_block_size);

        let delta_x = Vector3::x() * (self.camera.viewport_width * self.cur_block_size as f32 / self.canvas.get_width() as f32);
        let delta_y = -Vector3::y() * (self.camera.viewport_height * self.cur_block_size as f32 / self.canvas.get_height() as f32);

        let start_pos = self.camera.upper_left_corner() + (delta_x + delta_y) / 2.0;

        for column in 0..points_x {
            for row in 0..points_y {
                if self.cur_block_size != self.max_block_size && row%3 == 1 && column%3 == 1 {
                    continue;
                }

                let hit_point = start_pos + (row as f32 * delta_y) + (column as f32 * delta_x); 

                let color = match self.ellipse.hit(hit_point.x, hit_point.y) {
                    HitRecord::Hit { z } => {
                        self.color_calculate(&Point3::new(hit_point.x, hit_point.y, z))
                    }

                    HitRecord::Miss => {
                        miss_color
                    }
                };

                self.canvas.draw_rectangle(
                    Point2::new(column * self.cur_block_size, row*self.cur_block_size),
                    Point2::new((column + 1)*self.cur_block_size - 1, (row + 1)*self.cur_block_size - 1),
                    color
                );
            }
        }

        if self.cur_block_size > 1 {
            self.cur_block_size /= 3;
        }
    }


    fn color_calculate(&self, pos: &Point3<f32>) -> Color {
        let coef = (CAMERA_CENTER - pos).normalize().dot(&self.ellipse.normal(pos)).powf(self.brightness);
        self.ellipse.color * coef
    }


    fn reset_blocks_size(&mut self) {
        self.cur_block_size = self.max_block_size;
    }


    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas.resize(width, height);
        self.reset_blocks_size();
    }


    pub fn rotate_ellipse(&mut self, axis: &UnitVector3<f32>, angle: f32) {
        self.ellipse.rotate(axis, angle);
        self.reset_blocks_size();
    }

    pub fn set_ellipsoid_a(&mut self, a: f32) {
        self.ellipse.set_a(a);
        self.reset_blocks_size();
    }

    pub fn set_ellipsoid_b(&mut self, b: f32) {
        self.ellipse.set_b(b);
        self.reset_blocks_size();
    }

    pub fn set_ellipsoid_c(&mut self, c: f32) {
        self.ellipse.set_c(c);
        self.reset_blocks_size();
    }

    pub fn set_brightness(&mut self, value: f32) {
        self.brightness = value;
        self.reset_blocks_size();
    }
}


#[cfg(test)]
mod various_tests {
    #[test]
    fn modula_test() {
        assert_eq!(false, 0 % 3 == 1);
        assert_eq!(true,  1 % 3 == 1);
        assert_eq!(false, 2 % 3 == 1);
        assert_eq!(false, 3 % 3 == 1);
        assert_eq!(true,  4 % 3 == 1);
        assert_eq!(false, 5 % 3 == 1);
        assert_eq!(false, 6 % 3 == 1)
    }
}