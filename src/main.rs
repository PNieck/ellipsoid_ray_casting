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

                fill_pixels(&mut pixels);

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


fn fill_pixels(pixels: &mut Pixels) {
    for pixel in pixels.frame_mut().chunks_exact_mut(4) {
        pixel[0] = 0x5eu8;
        pixel[1] = 0x48u8;
        pixel[2] = 0xe8u8;
        pixel[3] = 0xffu8;
    }
}