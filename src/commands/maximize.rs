use anyhow::Result;
use crate::automation::WindowController;
use crate::config::Config;

pub fn execute(_config: &Config) -> Result<()> {
    println!("Maximizing active window...");
    
    match WindowController::maximize_active_window() {
        Ok(()) => {
            println!("✓ Window maximized successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Failed to maximize window: {}", e);
            eprintln!("Note: This command requires 'wmctrl' to be installed on Linux");
            eprintln!("Install with: sudo apt install wmctrl");
            Err(e)
        }
    }
}