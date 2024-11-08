use super::Tool;
use crate::engine::Vertex;
use glam::Vec2;

#[derive(Debug)]
pub struct Eraser {
    points: Vec<Vec2>,
    size: f32,
}

impl Eraser {
    pub fn new(size: f32) -> Self {
        Self {
            points: Vec::new(),
            size,
        }
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }
}

impl Tool for Eraser {
    fn on_mouse_down(&mut self, position: Vec2) {
        self.points.push(position);
    }

    fn on_mouse_move(&mut self, position: Vec2) -> Vec<Vertex> {
        self.points.push(position);
        self.get_vertices()
    }

    fn on_mouse_up(&mut self, _position: Vec2) -> Vec<Vertex> {
        self.get_vertices()
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        
        for pos in &self.points {
            let size = self.size;
            
            let top_left = [pos.x - size, pos.y + size];
            let top_right = [pos.x + size, pos.y + size];
            let bottom_left = [pos.x - size, pos.y - size];
            let bottom_right = [pos.x + size, pos.y - size];

            // First triangle
            vertices.push(Vertex {
                position: top_left,
                color: [1.0, 1.0, 1.0, 1.0], // White color for eraser
            });
            vertices.push(Vertex {
                position: bottom_left,
                color: [1.0, 1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: top_right,
                color: [1.0, 1.0, 1.0, 1.0],
            });

            // Second triangle
            vertices.push(Vertex {
                position: bottom_left,
                color: [1.0, 1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: bottom_right,
                color: [1.0, 1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: top_right,
                color: [1.0, 1.0, 1.0, 1.0],
            });
        }

        vertices
    }

    fn set_color(&mut self, _color: [f32; 4]) {
        // Eraser doesn't use color
    }

    fn set_size(&mut self, size: f32) {
        self.size = size;
    }
} 