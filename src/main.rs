extern crate nalgebra as na;

use ellipsoid_ray_casting::Scene;
use na::{vector, Point2, UnitVector3, Vector2, Vector3};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut scene = Scene::new(&window);
    scene.update();

    let mut mouse_pressed = false;
    //let mut prev_mouse_pos = Point2::origin();
    let mut cur_mouse_pos = Point2::origin();
    //let mut mouse_move_vec = Vector2::zeros();

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
                scene.render();
                window.request_redraw();
            },
            Event::WindowEvent { 
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                scene.resize(new_size.width, new_size.height);
                scene.update();
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, ..},
                ..
            } => {
                mouse_pressed = true;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, ..},
                ..
            } => {
                mouse_pressed = false;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let prev_mouse_pos = cur_mouse_pos;
                cur_mouse_pos.x = position.x;
                cur_mouse_pos.y = -position.y;

                let mouse_move_vec = cur_mouse_pos - prev_mouse_pos;

                if mouse_pressed {
                    let axis =  UnitVector3::new_normalize(Vector3::new(mouse_move_vec.y as f32, mouse_move_vec.x as f32, 0.0));
                    scene.rotate_ellipse(&axis, mouse_move_vec.norm() as f32 * 0.04);

                    scene.update();
                    scene.render();
                }
            }
            _ => ()
        }
    }).unwrap();
}
