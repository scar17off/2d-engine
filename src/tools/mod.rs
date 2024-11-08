use glam::Vec2;
use crate::engine::Vertex;

pub trait Tool {
    fn on_mouse_down(&mut self, position: Vec2);
    fn on_mouse_move(&mut self, position: Vec2) -> Vec<Vertex>;
    fn on_mouse_up(&mut self, position: Vec2) -> Vec<Vertex>;
    fn get_vertices(&self) -> Vec<Vertex>;
    fn set_color(&mut self, color: [f32; 4]);
    fn set_size(&mut self, size: f32);
}

pub mod brush;
pub mod eraser;
pub mod line;
pub mod rectangle;

use brush::Brush;
use eraser::Eraser;
use line::Line;
use rectangle::Rectangle;

#[derive(Debug)]
pub enum ActiveTool {
    Brush(Brush),
    Eraser(Eraser),
    Line(Line),
    Rectangle(Rectangle),
}

impl Tool for ActiveTool {
    fn on_mouse_down(&mut self, position: Vec2) {
        match self {
            ActiveTool::Brush(t) => t.on_mouse_down(position),
            ActiveTool::Eraser(t) => t.on_mouse_down(position),
            ActiveTool::Line(t) => t.on_mouse_down(position),
            ActiveTool::Rectangle(t) => t.on_mouse_down(position),
        }
    }

    fn on_mouse_move(&mut self, position: Vec2) -> Vec<Vertex> {
        match self {
            ActiveTool::Brush(t) => t.on_mouse_move(position),
            ActiveTool::Eraser(t) => t.on_mouse_move(position),
            ActiveTool::Line(t) => t.on_mouse_move(position),
            ActiveTool::Rectangle(t) => t.on_mouse_move(position),
        }
    }

    fn on_mouse_up(&mut self, position: Vec2) -> Vec<Vertex> {
        match self {
            ActiveTool::Brush(t) => t.on_mouse_up(position),
            ActiveTool::Eraser(t) => t.on_mouse_up(position),
            ActiveTool::Line(t) => t.on_mouse_up(position),
            ActiveTool::Rectangle(t) => t.on_mouse_up(position),
        }
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        match self {
            ActiveTool::Brush(t) => t.get_vertices(),
            ActiveTool::Eraser(t) => t.get_vertices(),
            ActiveTool::Line(t) => t.get_vertices(),
            ActiveTool::Rectangle(t) => t.get_vertices(),
        }
    }

    fn set_color(&mut self, color: [f32; 4]) {
        match self {
            ActiveTool::Brush(t) => t.set_color(color),
            ActiveTool::Line(t) => t.set_color(color),
            ActiveTool::Rectangle(t) => t.set_color(color),
            _ => {},
        }
    }

    fn set_size(&mut self, size: f32) {
        match self {
            ActiveTool::Brush(t) => t.set_size(size),
            ActiveTool::Eraser(t) => t.set_size(size),
            ActiveTool::Line(t) => t.set_size(size),
            _ => {},
        }
    }
}