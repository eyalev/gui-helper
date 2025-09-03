use anyhow::Result;
use std::process::Command;

pub struct WindowController;

impl WindowController {
    pub fn maximize_active_window() -> Result<()> {
        // Use wmctrl to properly maximize the active window with window manager state
        let maximize_output = Command::new("wmctrl")
            .args(&["-r", ":ACTIVE:", "-b", "add,maximized_vert,maximized_horz"])
            .output()?;
        
        if !maximize_output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to maximize window: {}",
                String::from_utf8_lossy(&maximize_output.stderr)
            ));
        }
        
        Ok(())
    }
    
    pub fn maximize_window_by_name(window_name: &str) -> Result<()> {
        // Use wmctrl to maximize a window by name
        let maximize_output = Command::new("wmctrl")
            .args(&["-r", window_name, "-b", "add,maximized_vert,maximized_horz"])
            .output()?;
        
        if !maximize_output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to maximize window '{}': {}",
                window_name,
                String::from_utf8_lossy(&maximize_output.stderr)
            ));
        }
        
        Ok(())
    }
    
    pub fn restore_window() -> Result<()> {
        // Use wmctrl to restore (un-maximize) the currently active window
        let restore_output = Command::new("wmctrl")
            .args(&["-r", ":ACTIVE:", "-b", "remove,maximized_vert,maximized_horz"])
            .output()?;
        
        if !restore_output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to restore window: {}",
                String::from_utf8_lossy(&restore_output.stderr)
            ));
        }
        
        Ok(())
    }
}