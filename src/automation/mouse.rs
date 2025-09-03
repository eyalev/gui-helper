use anyhow::Result;
use enigo::{Enigo, Mouse, Button, Coordinate};
use std::thread;
use std::time::Duration;

pub struct MouseController {
    enigo: Enigo,
}

impl MouseController {
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&enigo::Settings::default())?;
        Ok(Self { enigo })
    }
    
    pub fn click(&mut self, x: u32, y: u32, delay_ms: u64, double_click: bool) -> Result<()> {
        // Add delay before clicking
        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms));
        }
        
        // Move mouse to position
        self.enigo.move_mouse(x as i32, y as i32, Coordinate::Abs)?;
        
        // Small delay after moving mouse
        thread::sleep(Duration::from_millis(10));
        
        if double_click {
            // Perform double click
            self.enigo.button(Button::Left, enigo::Direction::Click)?;
            thread::sleep(Duration::from_millis(50)); // Small delay between clicks
            self.enigo.button(Button::Left, enigo::Direction::Click)?;
        } else {
            // Perform single click
            self.enigo.button(Button::Left, enigo::Direction::Click)?;
        }
        
        Ok(())
    }
    
    pub fn get_mouse_position(&mut self) -> Result<(i32, i32)> {
        self.enigo.location().map_err(|e| anyhow::anyhow!("Failed to get mouse position: {}", e))
    }
    
    pub fn move_to(&mut self, x: u32, y: u32) -> Result<()> {
        self.enigo.move_mouse(x as i32, y as i32, Coordinate::Abs)?;
        Ok(())
    }
}