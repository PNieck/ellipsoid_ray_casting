extern crate nalgebra as na;

use na::{Point3, Vector3, point};


pub const CAMERA_CENTER: Point3<f32> = point![0.0, 0.0, -20.0];


pub struct Camera {
    pub viewport_width: f32,
    pub viewport_height: f32
}


pub struct PointsIter {
    start_pos: Point3<f32>,

    cur_x: u32,
    cur_y: u32,

    img_width: u32,
    img_height: u32,

    delta_x: Vector3<f32>,
    delta_y: Vector3<f32>
}


impl Camera {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Camera {
        Camera{viewport_width, viewport_height}
    }


    pub fn upper_left_corner(&self) -> Point3<f32> {
        let translation = Vector3::y() * (self.viewport_height/2.0) + Vector3::x() * (-self.viewport_width/2.0);

        CAMERA_CENTER + translation
    }


    pub fn get_points_iterator(&self, img_width: u32, img_height: u32) -> PointsIter {
        PointsIter::new(self, img_width, img_height)
    }
}


impl PointsIter {
    fn new(camera: &Camera, img_width: u32, img_height: u32) -> PointsIter {
        let delta_x = Vector3::x() * camera.viewport_width / (img_width ) as f32;
        let delta_y = -Vector3::y() * camera.viewport_height / (img_height ) as f32;

        PointsIter {
            start_pos: camera.upper_left_corner() + (delta_x + delta_y)/2.0,
            cur_x: 0,
            cur_y: 0,
            img_width,
            img_height,
            delta_x,
            delta_y,
        }
    }
}


impl Iterator for PointsIter {
    type Item = Point3<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_y >= self.img_height {
            return None;
        }

        let result = self.start_pos + self.delta_x * self.cur_x as f32 + self.delta_y * self.cur_y as f32;

        self.cur_x += 1;

        if self.cur_x >= self.img_width {
            self.cur_y += 1;
            self.cur_x = 0;
        }

        Option::Some(result)
    }
}


#[cfg(test)]
mod camera_tests {
    use super::*;

    #[test]
    fn upper_left_corner_square_viewport() {
        let camera = Camera::new(3.0, 3.0);

        assert_eq!(point![-1.5, 1.5, CAMERA_CENTER.z], camera.upper_left_corner());
    }

    #[test]
    fn upper_left_corner_not_square_viewport() {
        let camera = Camera::new(4.0, 3.0);

        assert_eq!(point![-2.0, 1.5, CAMERA_CENTER.z], camera.upper_left_corner());
    }
}


#[cfg(test)]
mod points_iterator_tests {
    use super::*;


    #[test]
    fn single_pixel_squared() {
        let camera = Camera::new(3.0, 3.0);

        let mut iter = camera.get_points_iterator(1, 1);

        assert_eq!(Some(point![0.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(None, iter.next());
    }


    #[test]
    fn single_pixel_not_squared() {
        let camera = Camera::new(5.0, 3.0);

        let mut iter = camera.get_points_iterator(1, 1);

        assert_eq!(Some(point![0.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(None, iter.next());
    }


    #[test]
    fn simple_iteration_3x3() {
        let camera = Camera::new(3.0, 3.0);

        let mut iter = camera.get_points_iterator(3, 3);

        // First row
        assert_eq!(Some(point![-1.0, 1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 0.0, 1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 1.0, 1.0, CAMERA_CENTER.z]), iter.next());

        // Second row
        assert_eq!(Some(point![-1.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 0.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 1.0, 0.0, CAMERA_CENTER.z]), iter.next());

        // Third row
        assert_eq!(Some(point![-1.0, -1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 0.0, -1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 1.0, -1.0, CAMERA_CENTER.z]), iter.next());

        assert_eq!(None, iter.next());
    }


    #[test]
    fn simple_iteration_5x3() {
        let camera = Camera::new(5.0, 3.0);

        let mut iter = camera.get_points_iterator(5, 3);

        // First row
        assert_eq!(Some(point![-2.0, 1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![-1.0, 1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 0.0, 1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 1.0, 1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 2.0, 1.0, CAMERA_CENTER.z]), iter.next());

        // Second row
        assert_eq!(Some(point![-2.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![-1.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 0.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 1.0, 0.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 2.0, 0.0, CAMERA_CENTER.z]), iter.next());

        // Third row
        assert_eq!(Some(point![-2.0, -1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![-1.0, -1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 0.0, -1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 1.0, -1.0, CAMERA_CENTER.z]), iter.next());
        assert_eq!(Some(point![ 2.0, -1.0, CAMERA_CENTER.z]), iter.next());

        assert_eq!(None, iter.next());
    }
}