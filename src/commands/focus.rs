use anyhow::Result;
use crate::automation::WindowController;
use crate::config::Config;

pub fn execute(_config: &Config, window_name: Option<&str>, list: bool) -> Result<()> {
    if list {
        println!("Available windows:");
        match WindowController::list_windows() {
            Ok(windows) => {
                if windows.is_empty() {
                    println!("  No windows found");
                } else {
                    for (id, title) in windows {
                        println!("  [{}] {}", id, title);
                    }
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("✗ Failed to list windows: {}", e);
                eprintln!("Note: This command requires 'wmctrl' to be installed on Linux");
                eprintln!("Install with: sudo apt install wmctrl");
                Err(e)
            }
        }
    } else if let Some(window_name) = window_name {
        println!("Focusing on window matching: '{}'...", window_name);
        
        match WindowController::focus_window(window_name) {
            Ok(()) => {
                println!("✓ Window focused successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("✗ {}", e);
                Ok(()) // Don't propagate error for user-friendly message
            }
        }
    } else {
        eprintln!("Please provide a window name or use --list to see available windows");
        eprintln!("Usage: gui-helper focus <WINDOW> or gui-helper focus --list");
        Ok(())
    }
}