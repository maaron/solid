use solid_modeling::prelude::*;
use solid_modeling::evaluator::Evaluator2D;
use solid_modeling::renderer::Renderer2D;
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
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            evaluator: None,
            current_scene: 0,
            needs_update: true,
            zoom: 5.0,
            pan_x: 0.0,
            pan_y: 0.0,
            is_dragging: false,
            last_mouse_pos: None,
        }
    }

    async fn init_renderer(&mut self, window: std::sync::Arc<Window>) {
        let size = window.inner_size();
        let renderer = Renderer2D::new(window.clone(), size.width, size.height).await;
        let mut evaluator = Evaluator2D::new(size.width, size.height);

        // Set initial view based on zoom and pan
        Self::update_view_internal(&mut evaluator, self.zoom, self.pan_x, self.pan_y);

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.evaluator = Some(evaluator);
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
                    self.current_scene = (self.current_scene + 1) % 5;
                    self.needs_update = true;
                    println!("Scene: {}", self.current_scene);
                    true
                }
                KeyCode::Digit1 => {
                    self.current_scene = 0;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit2 => {
                    self.current_scene = 1;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit3 => {
                    self.current_scene = 2;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit4 => {
                    self.current_scene = 3;
                    self.needs_update = true;
                    true
                }
                KeyCode::Digit5 => {
                    self.current_scene = 4;
                    self.needs_update = true;
                    true
                }
                KeyCode::KeyR => {
                    // Reset view
                    self.zoom = 5.0;
                    self.pan_x = 0.0;
                    self.pan_y = 0.0;
                    self.update_view();
                    self.needs_update = true;
                    println!("View reset");
                    true
                }
                // Arrow keys for panning
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
                // +/- for zooming
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
                        if *y > 0.0 {
                            0.9
                        } else {
                            1.1
                        }
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        if pos.y > 0.0 {
                            0.9
                        } else {
                            1.1
                        }
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

                            // Convert pixel delta to world delta
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

    fn update(&mut self) {
        if self.needs_update {
            if let (Some(renderer), Some(evaluator)) = (&self.renderer, &self.evaluator) {
                let pixels = match self.current_scene {
                    0 => {
                        // Scene 1: Simple circle
                        let shape = circle(2.0);
                        evaluator.evaluate(&shape)
                    }
                    1 => {
                        // Scene 2: Union of two circles
                        let shape = circle(1.5)
                            .translate(-1.0, 0.0)
                            .union(circle(1.5).translate(1.0, 0.0));
                        evaluator.evaluate(&shape)
                    }
                    2 => {
                        // Scene 3: Smooth union (metaballs)
                        let shape = circle(1.5)
                            .translate(-1.0, 0.0)
                            .smooth_union(circle(1.5).translate(1.0, 0.0), 0.8);
                        evaluator.evaluate(&shape)
                    }
                    3 => {
                        // Scene 4: Difference (donut)
                        let shape = circle(2.5).subtract(circle(1.5));
                        evaluator.evaluate(&shape)
                    }
                    4 => {
                        // Scene 5: Complex composition
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
                };

                renderer.update_texture(&pixels);
                self.needs_update = false;
            }
        }
    }

    fn render(&self) -> Result<(), wgpu::SurfaceError> {
        if let Some(renderer) = &self.renderer {
            renderer.render()
        } else {
            Ok(())
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title("Solid Modeling - 2D Implicit Functions")
                .with_inner_size(winit::dpi::PhysicalSize::new(800, 800));

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

    println!("=== Solid Modeling Viewer ===");
    println!("\nControls:");
    println!("  SPACE      - Next scene");
    println!("  1-5        - Select specific scene");
    println!("  Mouse drag - Pan view");
    println!("  Scroll     - Zoom in/out");
    println!("  +/-        - Zoom in/out");
    println!("  Arrow keys - Pan view");
    println!("  R          - Reset view");
    println!("  ESC        - Quit");
    println!("\nScenes:");
    println!("  1: Simple circle");
    println!("  2: Union of two circles");
    println!("  3: Smooth union (metaballs)");
    println!("  4: Difference (donut)");
    println!("  5: Complex face composition");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
