use crate::grid::{OverviewSquare, ZoomSquare, ZoomArea};
use crate::config::{OverviewGridConfig, ZoomGridConfig};

pub struct GridRenderer;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_name(name: &str, opacity: f32) -> Self {
        let alpha = (opacity * 255.0) as u8;
        match name.to_lowercase().as_str() {
            "red" => Color { r: 255, g: 0, b: 0, a: alpha },
            "blue" => Color { r: 0, g: 0, b: 255, a: alpha },
            "green" => Color { r: 0, g: 255, b: 0, a: alpha },
            "yellow" => Color { r: 255, g: 255, b: 0, a: alpha },
            "white" => Color { r: 255, g: 255, b: 255, a: alpha },
            "black" => Color { r: 0, g: 0, b: 0, a: alpha },
            _ => Color { r: 255, g: 0, b: 0, a: alpha }, // Default to red
        }
    }
}

impl GridRenderer {
    pub fn render_overview_grid(
        frame: &mut [u8],
        width: u32,
        height: u32,
        squares: &[OverviewSquare],
        config: &OverviewGridConfig,
    ) {
        // Clear frame with transparent background
        Self::clear_frame(frame);
        
        let color = Color::from_name(&config.color, config.opacity);
        
        // Draw grid lines
        for square in squares {
            Self::draw_rectangle_outline(
                frame,
                width,
                height,
                square.x,
                square.y,
                square.width,
                square.height,
                config.thickness,
                &color,
            );
            
            if config.show_numbers {
                Self::draw_text_simple(
                    frame,
                    width,
                    height,
                    &square.id,
                    square.x + 5,
                    square.y + 5,
                    &color,
                );
            }
        }
    }
    
    pub fn render_zoom_grid(
        frame: &mut [u8],
        width: u32,
        height: u32,
        zoom_area: &ZoomArea,
        squares: &[ZoomSquare],
        config: &ZoomGridConfig,
    ) {
        // Clear frame with transparent background
        Self::clear_frame(frame);
        
        let color = Color::from_name(&config.color, config.opacity);
        
        // Draw zoom area border (thicker)
        Self::draw_rectangle_outline(
            frame,
            width,
            height,
            zoom_area.x,
            zoom_area.y,
            zoom_area.width,
            zoom_area.height,
            config.thickness + 2,
            &color,
        );
        
        // Draw grid squares
        for square in squares {
            Self::draw_rectangle_outline(
                frame,
                width,
                height,
                square.abs_x,
                square.abs_y,
                square.width,
                square.height,
                config.thickness,
                &color,
            );
            
            if config.show_numbers {
                Self::draw_text_simple(
                    frame,
                    width,
                    height,
                    &square.id.to_string(),
                    square.abs_x + 2,
                    square.abs_y + 2,
                    &color,
                );
            }
        }
    }
    
    fn clear_frame(frame: &mut [u8]) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0; // R
            pixel[1] = 0; // G
            pixel[2] = 0; // B
            pixel[3] = 0; // A (transparent)
        }
    }
    
    fn draw_rectangle_outline(
        frame: &mut [u8],
        width: u32,
        height: u32,
        x: u32,
        y: u32,
        rect_width: u32,
        rect_height: u32,
        thickness: u32,
        color: &Color,
    ) {
        // Top edge
        Self::draw_horizontal_line(frame, width, height, x, y, rect_width, thickness, color);
        // Bottom edge  
        Self::draw_horizontal_line(frame, width, height, x, y + rect_height - thickness, rect_width, thickness, color);
        // Left edge
        Self::draw_vertical_line(frame, width, height, x, y, rect_height, thickness, color);
        // Right edge
        Self::draw_vertical_line(frame, width, height, x + rect_width - thickness, y, rect_height, thickness, color);
    }
    
    fn draw_horizontal_line(
        frame: &mut [u8],
        width: u32,
        height: u32,
        x: u32,
        y: u32,
        length: u32,
        thickness: u32,
        color: &Color,
    ) {
        for dy in 0..thickness {
            for dx in 0..length {
                let px = x + dx;
                let py = y + dy;
                if px < width && py < height {
                    let index = ((py * width + px) * 4) as usize;
                    if index + 3 < frame.len() {
                        frame[index] = color.r;
                        frame[index + 1] = color.g;
                        frame[index + 2] = color.b;
                        frame[index + 3] = color.a;
                    }
                }
            }
        }
    }
    
    fn draw_vertical_line(
        frame: &mut [u8],
        width: u32,
        height: u32,
        x: u32,
        y: u32,
        length: u32,
        thickness: u32,
        color: &Color,
    ) {
        for dx in 0..thickness {
            for dy in 0..length {
                let px = x + dx;
                let py = y + dy;
                if px < width && py < height {
                    let index = ((py * width + px) * 4) as usize;
                    if index + 3 < frame.len() {
                        frame[index] = color.r;
                        frame[index + 1] = color.g;
                        frame[index + 2] = color.b;
                        frame[index + 3] = color.a;
                    }
                }
            }
        }
    }
    
    // Simple text rendering (just draw the text as basic pixels)
    // For a production system, you'd want to use a proper font rendering library
    fn draw_text_simple(
        frame: &mut [u8],
        width: u32,
        height: u32,
        text: &str,
        x: u32,
        y: u32,
        color: &Color,
    ) {
        // For now, just draw a small rectangle as placeholder for text
        // In a real implementation, you'd render actual font glyphs
        let text_width = text.len() as u32 * 6; // Rough approximation
        let text_height = 8;
        
        Self::draw_rectangle_outline(frame, width, height, x, y, text_width, text_height, 1, color);
    }
}