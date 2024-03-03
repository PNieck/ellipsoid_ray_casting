extern crate nalgebra as na;

use crate::math::quadratic_equation;
use na::{Matrix4, Vector4, Point3};


pub struct Ellipse {
    ellipse_m: Matrix4<f32>,
    model_m: Matrix4<f32>,

    result_m: Matrix4<f32>,
}


pub enum HitRecord {
    Hit{z: f32},
    Miss
}



impl Ellipse {
    pub fn new(a: f32, b: f32, c: f32, pos: Point3<f32>) -> Ellipse {
        let mut res = Ellipse {
            ellipse_m: Matrix4::from_diagonal(&Vector4::new(a, b, c, -1.0_f32)),
            model_m: Matrix4::new_translation(&pos.coords),

            result_m: Matrix4::zeros()
        };

        res.update_matrices();

        res
    }


    fn update_matrices(&mut self) {
        let model_inv = self.model_m.try_inverse().unwrap();

        self.result_m = model_inv.transpose() * self.ellipse_m * model_inv;
    }


    pub fn hit(&self, x: f32, y:f32) -> HitRecord {
        let m = &self.result_m;

        let a: f32 = m[(2, 2)];

        let b: f32 = (m[(2, 0)] + m[(0, 2)]) * x +
                     (m[(2, 1)] + m[(1, 2)]) * y +
                     (m[(3, 2)] + m[(2, 3)]);

        let c: f32 = m[(0, 0)] * x * x +
                     m[(1, 1)] * y * y +
                     m[(3, 3)] +
                     (m[(3, 0)] + m[(0, 3)]) * x +
                     (m[(3, 1)] + m[(1, 3)]) * y +
                     (m[(1, 0)] + m[(0, 1)]) * x * y;
        
        match quadratic_equation::solve(a, b, c) {
            quadratic_equation::Solutions::Two(v1, v2) =>
                HitRecord::Hit { z: f32::min(v1, v2) },
            
            quadratic_equation::Solutions::One(v) => HitRecord::Hit { z: v },

            quadratic_equation::Solutions::None => HitRecord::Miss
        }
    }
}
