use std::ops::RangeInclusive;

use egui::{ClippedPrimitive, Context, TexturesDelta, Ui};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Gui {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI
    pub state: GuiState,
}


pub struct GuiState {
    /// Only show the egui window when true.
    window_open: bool,

    pub old_a: f32,
    pub a: f32,

    pub old_b: f32,
    pub b: f32,

    pub old_c: f32,
    pub c: f32,

    pub old_m: f32,
    pub m: f32,

    pub old_scale: f32,
    pub scale: f32,

    pub old_max_block_size: u32,
    pub max_block_size: u32,
}

impl Gui {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();
        let gui = GuiState::new();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            state: gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            self.state.ui(egui_ctx);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }

    pub fn uses_mouse(&self) -> bool {
        self.egui_ctx.is_pointer_over_area() || self.egui_ctx.is_using_pointer()
    }
}

impl GuiState {
    /// Create a `Gui`.
    fn new() -> Self {
        Self {
            window_open: true,

            old_a: 0.0,
            a: 2.0,

            old_b: 0.0,
            b: 1.0,

            old_c: 0.0,
            c: 3.0,

            old_m: 0.0,
            m: 1.0,

            old_scale: 0.0,
            scale: 1.0,

            old_max_block_size: 0,
            max_block_size: 27,
        }
    }

    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Ellipsoid ray casting")
            .open(&mut self.window_open)
            .show(ctx, |ui| {

                ui.label("Ellipsoid parameters");
                float_input("a:", &mut self.a, 0.0..=5.0, ui);
                float_input("b:", &mut self.b, 0.0..=5.0, ui);
                float_input("c:", &mut self.c, 0.0..=5.0, ui);
                float_input("scale:", &mut self.scale, 0.01..=5.0, ui);

                ui.separator();

                ui.label("Light options");
                float_input("m: ", &mut self.m, 0.0..=20.0, ui);

                ui.separator();

                egui::ComboBox::from_label("Select one!")
                    .selected_text(format!("{}", self.max_block_size))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.max_block_size, 1, "1");
                        ui.selectable_value(&mut self.max_block_size, 3, "3");
                        ui.selectable_value(&mut self.max_block_size, 9, "9");
                        ui.selectable_value(&mut self.max_block_size, 27, "27");
                        ui.selectable_value(&mut self.max_block_size, 81, "81");
                    }
                );
            });

        fn float_input(label: &str, value: &mut f32, range: RangeInclusive<f32>, ui: &mut Ui) {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::DragValue::new(value)
                    .speed(0.01)
                    .clamp_range(range));
            });
        }
    }
}
