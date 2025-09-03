use anyhow::Result;
use crate::automation::WindowController;
use crate::config::Config;
use crate::WindowOperation;

pub fn execute(_config: &Config, window_name: &str, operation: WindowOperation) -> Result<()> {
    match operation {
        WindowOperation::Maximize => {
            println!("Focusing and maximizing window: '{}'...", window_name);
            
            match WindowController::focus_and_maximize_window(window_name) {
                Ok(()) => {
                    println!("✓ Window '{}' focused and maximized successfully", window_name);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("✗ {}", e);
                    Ok(()) // Don't propagate error for user-friendly message
                }
            }
        }
        WindowOperation::Unmaximize => {
            println!("Focusing and restoring window: '{}'...", window_name);
            
            match WindowController::focus_and_unmaximize_window(window_name) {
                Ok(()) => {
                    println!("✓ Window '{}' focused and restored successfully", window_name);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("✗ {}", e);
                    Ok(()) // Don't propagate error for user-friendly message
                }
            }
        }
    }
}