use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use glam::Vec2;
use std::collections::VecDeque;

mod engine;
mod tools;

use tools::{Tool, ActiveTool, brush::Brush, eraser::Eraser, line::Line, rectangle::Rectangle};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ToolType {
    Brush,
    Eraser,
    Line,
    Rectangle,
}

struct Canvas {
    current_tool: ActiveTool,
    tool_type: ToolType,
    persistent_vertices: Vec<engine::Vertex>,
    current_vertices: Vec<engine::Vertex>,
    current_color: [f32; 4],
    brush_size: f32,
    is_drawing: bool,
    needs_update: bool,
    history: VecDeque<Vec<engine::Vertex>>,
    max_history: usize,
}

impl Canvas {
    fn new() -> Self {
        Self {
            current_tool: ActiveTool::Brush(Brush::new(0.01, [0.0, 0.0, 0.0, 1.0])),
            tool_type: ToolType::Brush,
            persistent_vertices: Vec::new(),
            current_vertices: Vec::new(),
            current_color: [0.0, 0.0, 0.0, 1.0],
            brush_size: 0.01,
            is_drawing: false,
            needs_update: false,
            history: VecDeque::with_capacity(20),
            max_history: 20,
        }
    }

    fn change_tool(&mut self, tool_type: ToolType) {
        if tool_type == self.tool_type {
            return;
        }

        self.tool_type = tool_type;
        self.current_tool = match tool_type {
            ToolType::Brush => ActiveTool::Brush(Brush::new(self.brush_size, self.current_color)),
            ToolType::Eraser => ActiveTool::Eraser(Eraser::new(self.brush_size)),
            ToolType::Line => ActiveTool::Line(Line::new(self.brush_size, self.current_color)),
            ToolType::Rectangle => ActiveTool::Rectangle(Rectangle::new(self.current_color)),
        };
    }

    fn set_color(&mut self, color: [f32; 4]) {
        self.current_color = color;
        self.current_tool.set_color(color);
    }

    fn set_brush_size(&mut self, size: f32) {
        self.brush_size = size;
        self.current_tool.set_size(size);
    }

    fn on_mouse_down(&mut self, position: Vec2) {
        self.is_drawing = true;
        self.current_tool.on_mouse_down(position);
        self.current_vertices = self.current_tool.get_vertices();
        self.needs_update = true;
    }

    fn on_mouse_move(&mut self, position: Vec2) {
        if self.is_drawing {
            self.current_tool.on_mouse_move(position);
            self.current_vertices = self.current_tool.get_vertices();
            self.needs_update = true;
        }
    }

    fn on_mouse_up(&mut self, position: Vec2) {
        if self.is_drawing {
            let new_vertices = self.current_tool.on_mouse_up(position);
            self.persistent_vertices.extend(new_vertices);
            self.current_vertices.clear();
            self.save_state();
            self.needs_update = true;
        }
        self.is_drawing = false;
    }

    fn save_state(&mut self) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(self.persistent_vertices.clone());
    }

    fn undo(&mut self) {
        if let Some(previous_state) = self.history.pop_back() {
            self.persistent_vertices = previous_state;
            self.needs_update = true;
        }
    }

    fn clear(&mut self) {
        self.save_state();
        self.persistent_vertices.clear();
        self.current_vertices.clear();
        self.needs_update = true;
    }

    fn get_all_vertices(&self) -> Vec<engine::Vertex> {
        let mut all_vertices = self.persistent_vertices.clone();
        all_vertices.extend(self.current_vertices.clone());
        all_vertices
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Paint")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let mut renderer = pollster::block_on(engine::Renderer2D::new(&window));
    let mut canvas = Canvas::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            if renderer.egui_state.on_event(&renderer.egui_ctx, event).consumed {
                return;
            }
        }

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position, .. } => {
                    if canvas.is_drawing {
                        let size = window.inner_size();
                        let x = (position.x / size.width as f64) * 2.0 - 1.0;
                        let y = -((position.y / size.height as f64) * 2.0 - 1.0);
                        canvas.on_mouse_move(Vec2::new(x as f32, y as f32));
                        canvas.needs_update = true;
                    }
                },

                WindowEvent::MouseInput {
                    state,
                    button: MouseButton::Left,
                    ..
                } => {
                    if let Ok(position) = window.inner_position() {
                        let size = window.inner_size();
                        let x = (position.x as f64 / size.width as f64) * 2.0 - 1.0;
                        let y = -((position.y as f64 / size.height as f64) * 2.0 - 1.0);
                        if *state == ElementState::Pressed {
                            canvas.on_mouse_down(Vec2::new(x as f32, y as f32));
                        } else {
                            canvas.on_mouse_up(Vec2::new(x as f32, y as f32));
                        }
                    }
                },

                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                    ..
                } => {
                    canvas.set_color(match key {
                        VirtualKeyCode::Key1 => [1.0, 0.0, 0.0, 1.0],
                        VirtualKeyCode::Key2 => [0.0, 1.0, 0.0, 1.0],
                        VirtualKeyCode::Key3 => [0.0, 0.0, 1.0, 1.0],
                        VirtualKeyCode::Key4 => [1.0, 1.0, 0.0, 1.0],
                        VirtualKeyCode::Key5 => [1.0, 0.0, 1.0, 1.0],
                        VirtualKeyCode::Key6 => [0.0, 0.0, 0.0, 1.0],
                        _ => canvas.current_color,
                    });
                },

                WindowEvent::Resized(physical_size) => {
                    renderer.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(**new_inner_size);
                },
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let raw_input = renderer.egui_state.take_egui_input(&window);
                
                // Store if we need to clear canvas
                let mut should_clear_canvas = false;
                
                let output = renderer.egui_ctx.run(raw_input, |ctx| {
                    egui::SidePanel::left("Tools").show(ctx, |ui| {
                        ui.heading("Tools");
                        
                        let old_tool = canvas.tool_type;
                        ui.radio_value(&mut canvas.tool_type, ToolType::Brush, "🖌 Brush");
                        ui.radio_value(&mut canvas.tool_type, ToolType::Eraser, "⌫ Eraser");
                        ui.radio_value(&mut canvas.tool_type, ToolType::Line, "📏 Line");
                        ui.radio_value(&mut canvas.tool_type, ToolType::Rectangle, "⬜ Rectangle");
                        
                        if old_tool != canvas.tool_type {
                            canvas.change_tool(canvas.tool_type);
                        }
                        
                        ui.separator();
                        
                        let old_size = canvas.brush_size;
                        ui.add(egui::Slider::new(&mut canvas.brush_size, 0.001..=0.05)
                            .text("Brush Size"));
                        if old_size != canvas.brush_size {
                            canvas.set_brush_size(canvas.brush_size);
                        }
                        
                        ui.separator();
                        
                        ui.heading("Colors");
                        let mut color = [
                            canvas.current_color[0],
                            canvas.current_color[1],
                            canvas.current_color[2],
                        ];
                        
                        if ui.color_edit_button_rgb(&mut color).changed() {
                            canvas.set_color([
                                color[0],
                                color[1],
                                color[2],
                                1.0,
                            ]);
                        }
                        
                        ui.horizontal(|ui| {
                            for color in &[
                                [1.0, 0.0, 0.0, 1.0],
                                [0.0, 1.0, 0.0, 1.0],
                                [0.0, 0.0, 1.0, 1.0],
                                [0.0, 0.0, 0.0, 1.0],
                            ] {
                                if ui.button("⬤").clicked() {
                                    canvas.set_color(*color);
                                }
                            }
                        });
                        
                        ui.separator();
                        
                        if ui.button("Clear Canvas").clicked() {
                            should_clear_canvas = true;
                        }
                    });
                });

                // Handle canvas clearing outside the egui closure
                if should_clear_canvas {
                    canvas.clear();
                    canvas.needs_update = true;
                }

                // Update vertices if needed
                if canvas.needs_update {
                    renderer.update_vertices(&canvas.get_all_vertices());
                    canvas.needs_update = false;
                }

                let primitives = renderer.egui_ctx.tessellate(output.shapes);
                
                match renderer.render(&window, primitives, output.textures_delta) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }

                renderer.egui_state.handle_platform_output(&window, &renderer.egui_ctx, output.platform_output);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
