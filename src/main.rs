extern crate nalgebra as na;

use ellipsoid_ray_casting::Scene;
use na::{Point2, UnitVector3, Vector3};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};
use winit::dpi::LogicalSize;


mod ui;


fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(600 as f64, 600 as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + egui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut scene = Scene::new(&window);
    scene.update();
    
    let window_size = window.inner_size();
    let mut gui = ui::Gui::new(&event_loop, window_size.width, window_size.height, window.scale_factor() as f32, scene.canvas.pixels());

    let mut mouse_pressed = false;
    let mut cur_mouse_pos = Point2::origin();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent { event, .. } => {
                match &event {
                    WindowEvent::Resized(new_size) => {
                        scene.resize(new_size.width, new_size.height);
                        gui.resize(new_size.width, new_size.height);
                        scene.update();
                    }

                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                        gui.scale_factor(*scale_factor);
                        scene.resize(new_inner_size.width, new_inner_size.height);
                        gui.resize(new_inner_size.width, new_inner_size.height);
                    }

                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                        mouse_pressed = true;
                    }

                    WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                        mouse_pressed = false;
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let prev_mouse_pos = cur_mouse_pos;
                        cur_mouse_pos.x = position.x;
                        cur_mouse_pos.y = -position.y;
        
                        let mouse_move_vec = cur_mouse_pos - prev_mouse_pos;
        
                        if mouse_pressed && !gui.uses_mouse() {
                            let axis =  UnitVector3::new_normalize(Vector3::new(mouse_move_vec.y as f32, mouse_move_vec.x as f32, 0.0));
                            scene.rotate_ellipse(&axis, mouse_move_vec.norm() as f32 * 0.04);
                        }
                    }

                    _ => ()
                }

                gui.handle_event(&event);

                window.request_redraw();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                handle_user_input(&mut scene, &mut gui);
                scene.update();
                gui.prepare(&window);

                scene.canvas.pixels().render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    gui.render(encoder, render_target, context);

                    Ok(())
                }).expect("Error while buffer rendering");
            }
            _ => ()
        }
    });
}


fn handle_user_input(scene: &mut Scene, gui: &mut ui::Gui) {
    if gui.state.old_a != gui.state.a {
        scene.set_ellipsoid_a(1.0 / (gui.state.a * gui.state.a));
        gui.state.old_a = gui.state.a;
    }

    if gui.state.old_b != gui.state.b {
        scene.set_ellipsoid_b(1.0 / (gui.state.b * gui.state.b));
        gui.state.old_b = gui.state.b;
    }

    if gui.state.old_c != gui.state.c {
        scene.set_ellipsoid_c(1.0 / (gui.state.c * gui.state.c));
        gui.state.old_c = gui.state.c;
    }

    if gui.state.old_m != gui.state.m {
        scene.set_brightness(gui.state.m);
        gui.state.old_m = gui.state.m;
    }
}
