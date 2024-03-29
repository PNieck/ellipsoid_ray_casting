extern crate nalgebra as na;

use ellipsoid_ray_casting::Scene;
use na::{Point2, Vector3};
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
            .with_title("Ellipsoid ray casting")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut scene = Scene::new(&window);
    scene.update();
    
    let window_size = window.inner_size();
    let mut gui = ui::Gui::new(&event_loop, window_size.width, window_size.height, window.scale_factor() as f32, scene.canvas.pixels());

    let mut mouse_left_pressed = false;
    let mut mouse_middle_presed = false;
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
                        mouse_left_pressed = true;
                    }

                    WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                        mouse_left_pressed = false;
                    }

                    WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Middle, .. } => {
                        mouse_middle_presed = true;
                    }

                    WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Middle, .. } => {
                        mouse_middle_presed = false;
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let prev_mouse_pos = cur_mouse_pos;
                        cur_mouse_pos.x = position.x;
                        cur_mouse_pos.y = position.y;
        
                        let mouse_move_vec = cur_mouse_pos - prev_mouse_pos;
        
                        if mouse_left_pressed && !gui.uses_mouse() {
                            scene.rotate_ellipse(-mouse_move_vec.y as f32 * 0.02, -mouse_move_vec.x as f32 * 0.02, 0.0);
                        }

                        if mouse_middle_presed && !gui.uses_mouse() {
                            scene.move_ellipse(&Vector3::new(mouse_move_vec.x as f32 * 0.001, -mouse_move_vec.y as f32 * 0.001, 0.0))
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

    if gui.state.max_block_size != gui.state.old_max_block_size {
        scene.set_max_block_size(gui.state.max_block_size);
        gui.state.old_max_block_size = gui.state.max_block_size;
    }

    if gui.state.scale != gui.state.old_scale {
        scene.set_ellipsoid_scale(gui.state.scale);
        gui.state.old_scale = gui.state.scale;
    }
}
