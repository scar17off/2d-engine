# 2D Paint Application (Unfinished)

A simple 2D paint application built with Rust using wgpu for rendering and egui for the UI.

## Current Features

- Basic drawing tools:
  - Brush
  - Eraser
  - Line
  - Rectangle
  (the tools are not fully implemented)
- Color picker
- Clear canvas

## Known Issues

The project is currently unfinished due to some unresolved issues:

1. Tool switching and brush size adjustment don't work properly - changes in the UI don't affect the actual drawing behavior
2. Mouse position tracking is imprecise, especially when moving the mouse fast
3. The eraser tool doesn't work
4. Line and rectangle preview (while dragging) is not implemented correctly
5. Brush size adjustment is not implemented

## Technical Details

The application uses:
- wgpu for GPU-accelerated rendering
- egui for the user interface
- A custom tool system with trait-based polymorphism

## Project Structure 
src/  
├── main.rs # Main application logic and UI  
├── engine.rs # Rendering engine using wgpu  
├── shader.wgsl # WGSL shader for rendering  
└── tools/ # Drawing tools implementation  
├── mod.rs # Tool trait and ActiveTool enum  
├── brush.rs # Brush tool implementation  
├── eraser.rs # Eraser tool implementation  
├── line.rs # Line tool implementation  
└── rectangle.rs # Rectangle tool implementation

## Future Work

If the issues can be resolved, planned features include:
- Undo/Redo functionality
- Layer support
- Save/Load functionality
- More drawing tools (circle, polygon, etc.)
- Advanced brush patterns
- Better eraser blending

## Dependencies
toml  
winit = "0.28"  
wgpu = "0.16"  
pollster = "0.3"  
bytemuck = { version = "1.13", features = ["derive"] }  
glam = "0.24"  
egui = "0.22"  
egui-winit = "0.22"  
egui-wgpu = "0.22"

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.