extern crate nalgebra as na;

use na::{Point3, UnitVector3};
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
        }
    }

    pub fn render(&self) {
        self.canvas.render();
    }

    pub fn update(&mut self) {
        let miss_color: Color = Color::from_rgb(120, 120, 120);

        let mut cam_points_iter = self.camera.get_points_iterator(self.canvas.get_width(), self.canvas.get_height());

        for row in 0..self.canvas.get_height() {
            for column in 0..self.canvas.get_width() {
                let hit_point = cam_points_iter.next().unwrap(); 

                match self.ellipse.hit(hit_point.x, hit_point.y) {
                    HitRecord::Hit { z } => {
                        let color = self.color_calculate(&Point3::new(hit_point.x, hit_point.y, z));
                        self.canvas.set_pixel(color, row, column);
                    }

                    HitRecord::Miss => {
                        self.canvas.set_pixel(miss_color, row, column);
                    }
                }
            }
        }
    }


    fn color_calculate(&self, pos: &Point3<f32>) -> Color {
        let coef = (CAMERA_CENTER - pos).normalize().dot(&self.ellipse.normal(pos)).powf(self.brightness);
        self.ellipse.color * coef
    }


    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas.resize(width, height)
    }


    pub fn rotate_ellipse(&mut self, axis: &UnitVector3<f32>, angle: f32) {
        self.ellipse.rotate(axis, angle);
    }

    pub fn set_ellipsoid_a(&mut self, a: f32) {
        self.ellipse.set_a(a);
    }

    pub fn set_ellipsoid_b(&mut self, b: f32) {
        self.ellipse.set_b(b);
    }

    pub fn set_ellipsoid_c(&mut self, c: f32) {
        self.ellipse.set_c(c);
    }

    pub fn set_brightness(&mut self, value: f32) {
        self.brightness = value;
    }
}
