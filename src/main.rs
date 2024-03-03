extern crate nalgebra as na;

use std::iter::zip;
use ellipsoid_ray_casting::objects::{camera::Camera, ellipse::{Ellipse, HitRecord}};
use na::Point3;
use winit::{
    event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};
use pixels::{Pixels, SurfaceTexture};




fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
    };

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let win_size = window.inner_size();
                fill_pixels(&mut pixels, win_size.width as usize, win_size.height as usize);

                pixels.render().unwrap();
                window.request_redraw();
            },
            Event::WindowEvent { 
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                pixels.resize_surface(new_size.width, new_size.height).unwrap();
                pixels.resize_buffer(new_size.width, new_size.height).unwrap();
            }
            _ => ()
        }
    }).unwrap();
}


fn fill_pixels(pixels: &mut Pixels, img_width: usize, img_height: usize) {

    let ellipse = Ellipse::new(1.0, 2.0, 3.0, Point3::new(0.0_f32, 0.0, 20.0));
    let camera = Camera::new(5.0, 5.0);

    for (pixel, cam_point) in zip(pixels.frame_mut().chunks_exact_mut(4), camera.get_points_iterator(img_width, img_height)) {
        match ellipse.hit(cam_point.x, cam_point.y) {
            HitRecord::Hit { z } => {
                pixel[0] = 00_u8;
                pixel[1] = 00_u8;
                pixel[2] = 00_u8;
                pixel[3] = 00_u8;
            }

            HitRecord::Miss => {
                pixel[0] = 255_u8;
                pixel[1] = 255_u8;
                pixel[2] = 255_u8;
                pixel[3] = 255_u8;
            }
        }
    }

    for pixel in pixels.frame_mut().chunks_exact_mut(4) {
        
    }
}
