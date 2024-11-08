use super::Tool;
use crate::engine::Vertex;
use glam::Vec2;

#[derive(Debug)]
pub struct Brush {
    points: Vec<(Vec2, [f32; 4])>,
    brush_size: f32,
    color: [f32; 4],
}

impl Brush {
    pub fn new(brush_size: f32, color: [f32; 4]) -> Self {
        Self {
            points: Vec::new(),
            brush_size,
            color,
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn set_size(&mut self, size: f32) {
        self.brush_size = size;
    }
}

impl Tool for Brush {
    fn on_mouse_down(&mut self, position: Vec2) {
        self.points.push((position, self.color));
    }

    fn on_mouse_move(&mut self, position: Vec2) -> Vec<Vertex> {
        self.points.push((position, self.color));
        self.get_vertices()
    }

    fn on_mouse_up(&mut self, _position: Vec2) -> Vec<Vertex> {
        self.get_vertices()
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        
        for (pos, color) in &self.points {
            let size = self.brush_size;
            
            let top_left = [pos.x - size, pos.y + size];
            let top_right = [pos.x + size, pos.y + size];
            let bottom_left = [pos.x - size, pos.y - size];
            let bottom_right = [pos.x + size, pos.y - size];

            // First triangle
            vertices.push(Vertex {
                position: top_left,
                color: *color,
            });
            vertices.push(Vertex {
                position: bottom_left,
                color: *color,
            });
            vertices.push(Vertex {
                position: top_right,
                color: *color,
            });

            // Second triangle
            vertices.push(Vertex {
                position: bottom_left,
                color: *color,
            });
            vertices.push(Vertex {
                position: bottom_right,
                color: *color,
            });
            vertices.push(Vertex {
                position: top_right,
                color: *color,
            });
        }

        vertices
    }

    fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    fn set_size(&mut self, size: f32) {
        self.brush_size = size;
    }
} 