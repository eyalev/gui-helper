use anyhow::Result;
use crate::automation::WindowController;
use crate::config::Config;

pub fn execute(_config: &Config) -> Result<()> {
    println!("Restoring active window from maximized state...");
    
    match WindowController::restore_window() {
        Ok(()) => {
            println!("✓ Window restored successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Failed to restore window: {}", e);
            eprintln!("Note: This command requires 'wmctrl' to be installed on Linux");
            eprintln!("Install with: sudo apt install wmctrl");
            Err(e)
        }
    }
}