use super::Tool;
use crate::engine::Vertex;
use glam::Vec2;

#[derive(Debug)]
pub struct Rectangle {
    start: Option<Vec2>,
    end: Option<Vec2>,
    color: [f32; 4],
}

impl Rectangle {
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            start: None,
            end: None,
            color,
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }
}

impl Tool for Rectangle {
    fn on_mouse_down(&mut self, position: Vec2) {
        self.start = Some(position);
        self.end = Some(position);
    }

    fn on_mouse_move(&mut self, position: Vec2) -> Vec<Vertex> {
        self.end = Some(position);
        self.get_vertices()
    }

    fn on_mouse_up(&mut self, position: Vec2) -> Vec<Vertex> {
        self.end = Some(position);
        let vertices = self.get_vertices();
        self.start = None;
        self.end = None;
        vertices
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => {
                let min_x = start.x.min(end.x);
                let max_x = start.x.max(end.x);
                let min_y = start.y.min(end.y);
                let max_y = start.y.max(end.y);

                vec![
                    // First triangle
                    Vertex {
                        position: [min_x, min_y],
                        color: self.color,
                    },
                    Vertex {
                        position: [max_x, min_y],
                        color: self.color,
                    },
                    Vertex {
                        position: [max_x, max_y],
                        color: self.color,
                    },
                    // Second triangle
                    Vertex {
                        position: [min_x, min_y],
                        color: self.color,
                    },
                    Vertex {
                        position: [max_x, max_y],
                        color: self.color,
                    },
                    Vertex {
                        position: [min_x, max_y],
                        color: self.color,
                    },
                ]
            }
            _ => Vec::new(),
        }
    }

    fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    fn set_size(&mut self, _size: f32) {
        // Rectangle doesn't use size
    }
} 