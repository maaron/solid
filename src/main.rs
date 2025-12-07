use solid_modeling::prelude::*;
use solid_modeling::evaluator::Evaluator2D;
use solid_modeling::renderer::Renderer2D;
use solid_modeling::lang::{parser::parse_program, eval::eval, env::Env};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId, WindowAttributes},
};

struct App {
    window: Option<std::sync::Arc<Window>>,
    renderer: Option<Renderer2D>,
    evaluator: Option<Evaluator2D>,
    current_scene: usize,
    needs_update: bool,
    // Camera state
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
    // Mouse state for dragging
    is_dragging: bool,
    last_mouse_pos: Option<(f32, f32)>,

    // egui state
    egui_ctx: egui::Context,
    egui_state: Option<egui_winit::State>,
    egui_renderer: Option<egui_wgpu::Renderer>,
    egui_output: Option<egui::FullOutput>,

    // Editor state
    code_text: String,
    parse_error: Option<String>,
    eval_error: Option<String>,
    use_editor_mode: bool,
    code_changed: bool,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            evaluator: None,
            current_scene: 0,
            needs_update: true,
            zoom: 10.0,
            pan_x: 0.0,
            pan_y: 0.0,
            is_dragging: false,
            last_mouse_pos: None,
            egui_ctx: egui::Context::default(),
            egui_state: None,
            egui_renderer: None,
            egui_output: None,
            code_text: r#"; Circle SDF
(fn p
  (- (length p) 5.0))"#.to_string(),
            parse_error: None,
            eval_error: None,
            use_editor_mode: true,
            code_changed: false,
        }
    }

    async fn init_renderer(&mut self, window: std::sync::Arc<Window>) {
        let size = window.inner_size();
        let renderer = Renderer2D::new(window.clone(), size.width, size.height).await;
        let mut evaluator = Evaluator2D::new(size.width, size.height);

        // Set initial view based on zoom and pan
        Self::update_view_internal(&mut evaluator, self.zoom, self.pan_x, self.pan_y);

        // Initialize egui
        let egui_ctx = self.egui_ctx.clone();
        let egui_state = egui_winit::State::new(
            egui_ctx,
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
            None, // max_texture_side
        );

        let device = renderer.device();
        let egui_renderer = egui_wgpu::Renderer::new(
            device,
            renderer.format(),
            Default::default(), // RendererOptions
        );

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.evaluator = Some(evaluator);
        self.egui_state = Some(egui_state);
        self.egui_renderer = Some(egui_renderer);

        // Parse initial code
        self.parse_and_evaluate();
        self.needs_update = true;
    }

    fn update_view_internal(evaluator: &mut Evaluator2D, zoom: f32, pan_x: f32, pan_y: f32) {
        let aspect = evaluator.width() as f32 / evaluator.height() as f32;
        let half_height = zoom;
        let half_width = zoom * aspect;

        evaluator.set_view(
            Vec2::new(pan_x - half_width, pan_y - half_height),
            Vec2::new(pan_x + half_width, pan_y + half_height),
        );
    }

    fn update_view(&mut self) {
        if let Some(evaluator) = &mut self.evaluator {
            Self::update_view_internal(evaluator, self.zoom, self.pan_x, self.pan_y);
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if let (Some(renderer), Some(evaluator)) = (&mut self.renderer, &mut self.evaluator) {
            renderer.resize(new_size.width, new_size.height);
            evaluator.resize(new_size.width, new_size.height);
            Self::update_view_internal(evaluator, self.zoom, self.pan_x, self.pan_y);
            self.needs_update = true;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        // Let egui handle input first
        if let (Some(window), Some(state)) = (&self.window, &mut self.egui_state) {
            let response = state.on_window_event(window, event);
            if response.consumed {
                return true;
            }
        }

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                KeyCode::Space => {
                    if !self.use_editor_mode {
                        self.current_scene = (self.current_scene + 1) % 5;
                        self.needs_update = true;
                        println!("Scene: {}", self.current_scene);
                    }
                    true
                }
                KeyCode::Tab => {
                    self.use_editor_mode = !self.use_editor_mode;
                    self.needs_update = true;
                    println!("Editor mode: {}", if self.use_editor_mode { "ON" } else { "OFF" });
                    true
                }
                KeyCode::Digit1 if !self.use_editor_mode => {
                    self.current_scene = 0;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit2 if !self.use_editor_mode => {
                    self.current_scene = 1;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit3 if !self.use_editor_mode => {
                    self.current_scene = 2;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit4 if !self.use_editor_mode => {
                    self.current_scene = 3;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit5 if !self.use_editor_mode => {
                    self.current_scene = 4;
                    self.needs_update = true;
                    true
                }
                KeyCode::KeyR => {
                    // Reset view
                    self.zoom = 10.0;
                    self.pan_x = 0.0;
                    self.pan_y = 0.0;
                    self.update_view();
                    self.needs_update = true;
                    println!("View reset");
                    true
                }
                KeyCode::ArrowLeft => {
                    self.pan_x -= self.zoom * 0.1;
                    self.update_view();
                    self.needs_update = true;
                    true
                }
                KeyCode::ArrowRight => {
                    self.pan_x += self.zoom * 0.1;
                    self.update_view();
                    self.needs_update = true;
                    true
                }
                KeyCode::ArrowUp => {
                    self.pan_y += self.zoom * 0.1;
                    self.update_view();
                    self.needs_update = true;
                    true
                }
                KeyCode::ArrowDown => {
                    self.pan_y -= self.zoom * 0.1;
                    self.update_view();
                    self.needs_update = true;
                    true
                }
                KeyCode::Equal | KeyCode::NumpadAdd => {
                    self.zoom *= 0.9;
                    self.update_view();
                    self.needs_update = true;
                    println!("Zoom: {:.2}", self.zoom);
                    true
                }
                KeyCode::Minus | KeyCode::NumpadSubtract => {
                    self.zoom *= 1.1;
                    self.update_view();
                    self.needs_update = true;
                    println!("Zoom: {:.2}", self.zoom);
                    true
                }
                _ => false,
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_factor = match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        if *y > 0.0 { 0.9 } else { 1.1 }
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        if pos.y > 0.0 { 0.9 } else { 1.1 }
                    }
                };
                self.zoom *= zoom_factor;
                self.update_view();
                self.needs_update = true;
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = (position.x as f32, position.y as f32);

                if self.is_dragging {
                    if let Some(last_pos) = self.last_mouse_pos {
                        if let Some(evaluator) = &self.evaluator {
                            let dx = pos.0 - last_pos.0;
                            let dy = pos.1 - last_pos.1;

                            let aspect = evaluator.width() as f32 / evaluator.height() as f32;
                            let world_width = self.zoom * 2.0 * aspect;
                            let world_height = self.zoom * 2.0;

                            self.pan_x -= dx * world_width / evaluator.width() as f32;
                            self.pan_y += dy * world_height / evaluator.height() as f32;
                        }

                        self.update_view();
                        self.needs_update = true;
                    }
                }

                self.last_mouse_pos = Some(pos);
                self.is_dragging
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.is_dragging = true;
                        true
                    }
                    ElementState::Released => {
                        self.is_dragging = false;
                        true
                    }
                }
            }
            _ => false,
        }
    }

    fn parse_and_evaluate(&mut self) {
        self.parse_error = None;
        self.eval_error = None;

        // Parse the code
        let expr = match parse_program(&self.code_text) {
            Ok(expr) => expr,
            Err(e) => {
                self.parse_error = Some(format!("{:?}", e));
                return;
            }
        };

        // Evaluate to get a function
        let env = Env::with_builtins();
        let _field_fn = match eval(&expr, &env) {
            Ok(val) => val,
            Err(e) => {
                self.eval_error = Some(format!("{:?}", e));
                return;
            }
        };

        // TODO: Wire up field_fn to evaluator
        // For now, evaluation success means no error
        self.needs_update = true;
    }

    fn update_egui(&mut self) {
        if let (Some(window), Some(state)) = (&self.window, &mut self.egui_state) {
            let raw_input = state.take_egui_input(window);

            let full_output = self.egui_ctx.run(raw_input, |ctx| {
                if self.use_editor_mode {
                    egui::SidePanel::left("editor")
                        .default_width(400.0)
                        .resizable(true)
                        .show(ctx, |ui| {
                            ui.heading("FieldCalc Editor");
                            ui.label("Write FieldCalc code to create SDFs");
                            ui.separator();

                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.code_text)
                                    .code_editor()
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(20)
                            );

                            if response.changed() {
                                self.code_changed = true;
                            }

                            ui.separator();

                            if let Some(err) = &self.parse_error {
                                ui.colored_label(egui::Color32::from_rgb(255, 100, 100),
                                    format!("Parse error: {}", err));
                            }
                            if let Some(err) = &self.eval_error {
                                ui.colored_label(egui::Color32::from_rgb(255, 100, 100),
                                    format!("Eval error: {}", err));
                            }

                            ui.separator();
                            ui.label("Examples:");
                            ui.monospace("; Circle\n(fn p\n  (- (length p) 5.0))");
                            ui.separator();
                            ui.monospace("; Union\n(fn p\n  (min\n    (- (length p) 5.0)\n    (- (length (vec_sub p (vec 8.0 0.0))) 3.0)))");
                        });

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Rendered Output");
                        ui.label("The SDF visualization appears here");
                    });
                } else {
                    egui::Window::new("Controls")
                        .default_pos([10.0, 10.0])
                        .show(ctx, |ui| {
                            ui.label("Press TAB to toggle editor mode");
                            ui.label("SPACE: Next scene");
                            ui.label("1-5: Select scene");
                            ui.label("R: Reset view");
                        });
                }
            });

            state.handle_platform_output(window, full_output.platform_output.clone());
            self.egui_output = Some(full_output);
        }
    }

    fn update(&mut self) {
        self.update_egui();

        // Parse and evaluate if code changed
        if self.code_changed {
            self.parse_and_evaluate();
            self.code_changed = false;
        }

        if self.needs_update {
            if let (Some(renderer), Some(evaluator)) = (&self.renderer, &self.evaluator) {
                let pixels = if self.use_editor_mode {
                    // TODO: Render from parsed FieldCalc code
                    // For now, show a default circle
                    let shape = circle(5.0);
                    evaluator.evaluate(&shape)
                } else {
                    match self.current_scene {
                        0 => {
                            let shape = circle(2.0);
                            evaluator.evaluate(&shape)
                        }
                        1 => {
                            let shape = circle(1.5)
                                .translate(-1.0, 0.0)
                                .union(circle(1.5).translate(1.0, 0.0));
                            evaluator.evaluate(&shape)
                        }
                        2 => {
                            let shape = circle(1.5)
                                .translate(-1.0, 0.0)
                                .smooth_union(circle(1.5).translate(1.0, 0.0), 0.8);
                            evaluator.evaluate(&shape)
                        }
                        3 => {
                            let shape = circle(2.5).subtract(circle(1.5));
                            evaluator.evaluate(&shape)
                        }
                        4 => {
                            let body = circle(2.0);
                            let eye_left = circle(0.3).translate(-0.7, 0.7);
                            let eye_right = circle(0.3).translate(0.7, 0.7);
                            let mouth = rectangle(1.2, 0.3)
                                .translate(0.0, -0.5)
                                .intersect(circle(1.8).translate(0.0, -1.0));

                            let shape = body.subtract(eye_left).subtract(eye_right).subtract(mouth);
                            evaluator.evaluate(&shape)
                        }
                        _ => vec![0u8; (renderer.width() * renderer.height() * 4) as usize],
                    }
                };

                renderer.update_texture(&pixels);
                self.needs_update = false;
            }
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let (Some(renderer), Some(egui_renderer), Some(window), Some(full_output)) =
            (&mut self.renderer, &mut self.egui_renderer, &self.window, &self.egui_output) {

            // Tessellate egui shapes into triangles
            let clipped_primitives = self.egui_ctx.tessellate(
                full_output.shapes.clone(),
                full_output.pixels_per_point,
            );

            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [renderer.width(), renderer.height()],
                pixels_per_point: window.scale_factor() as f32,
            };

            // Render wgpu content with egui overlay
            renderer.render_with_egui(egui_renderer, &clipped_primitives, &screen_descriptor, &full_output.textures_delta)
        } else {
            Ok(())
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title("Solid Modeling - Interactive FieldCalc Editor")
                .with_inner_size(winit::dpi::PhysicalSize::new(1200, 800));

            let window = std::sync::Arc::new(event_loop.create_window(window_attributes).unwrap());
            pollster::block_on(self.init_renderer(window));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if !self.input(&event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => event_loop.exit(),
                WindowEvent::Resized(physical_size) => {
                    self.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.update();
                    match self.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            if let Some(window) = &self.window {
                                self.resize(window.inner_size());
                            }
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    env_logger::init();

    println!("=== Solid Modeling - Interactive Editor ===");
    println!("\nControls:");
    println!("  TAB        - Toggle editor mode");
    println!("  Mouse drag - Pan view");
    println!("  Scroll     - Zoom in/out");
    println!("  +/-        - Zoom in/out");
    println!("  Arrow keys - Pan view");
    println!("  R          - Reset view");
    println!("  ESC        - Quit");
    println!("\nEditor Mode (TAB):");
    println!("  Write FieldCalc code in the left pane");
    println!("  See live rendering in the right pane");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
