extern crate nalgebra as na;

use ellipsoid_ray_casting::Scene;
use winit::{
    event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut scene = Scene::new(&window);
    scene.update();

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

                // Is it needed?
                scene.update();
            }
            _ => ()
        }
    }).unwrap();
}
