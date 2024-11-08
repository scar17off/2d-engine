use super::Tool;
use crate::engine::Vertex;
use glam::Vec2;

#[derive(Debug)]
pub struct Line {
    start: Option<Vec2>,
    end: Option<Vec2>,
    color: [f32; 4],
    thickness: f32,
}

impl Line {
    pub fn new(thickness: f32, color: [f32; 4]) -> Self {
        Self {
            start: None,
            end: None,
            color,
            thickness,
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn set_size(&mut self, size: f32) {
        self.thickness = size;
    }

    fn generate_line_vertices(&self, start: Vec2, end: Vec2) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let direction = (end - start).normalize();
        let perpendicular = Vec2::new(-direction.y, direction.x) * self.thickness;

        let v1 = start + perpendicular;
        let v2 = start - perpendicular;
        let v3 = end + perpendicular;
        let v4 = end - perpendicular;

        // First triangle
        vertices.push(Vertex {
            position: [v1.x, v1.y],
            color: self.color,
        });
        vertices.push(Vertex {
            position: [v2.x, v2.y],
            color: self.color,
        });
        vertices.push(Vertex {
            position: [v3.x, v3.y],
            color: self.color,
        });

        // Second triangle
        vertices.push(Vertex {
            position: [v2.x, v2.y],
            color: self.color,
        });
        vertices.push(Vertex {
            position: [v4.x, v4.y],
            color: self.color,
        });
        vertices.push(Vertex {
            position: [v3.x, v3.y],
            color: self.color,
        });

        vertices
    }
}

impl Tool for Line {
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
            (Some(start), Some(end)) => self.generate_line_vertices(start, end),
            _ => Vec::new(),
        }
    }

    fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    fn set_size(&mut self, size: f32) {
        self.thickness = size;
    }
} 