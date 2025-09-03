use anyhow::Result;
use crate::session::SessionManager;
use std::process::Command;
use std::path::Path;

pub fn execute(latest: bool, path: Option<String>) -> Result<()> {
    let screenshot_path = if latest {
        // Get the latest screenshot
        match SessionManager::get_latest_screenshot()? {
            Some(path) => path,
            None => {
                println!("No screenshots found in the screenshots directory");
                return Ok(());
            }
        }
    } else if let Some(custom_path) = path {
        // Use the provided path
        let path = Path::new(&custom_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("File not found: {}", custom_path));
        }
        path.to_path_buf()
    } else {
        return Err(anyhow::anyhow!("Either --latest or a specific path must be provided"));
    };
    
    println!("Opening screenshot: {:?}", screenshot_path);
    
    // Try different image viewers based on the platform
    let viewers = if cfg!(target_os = "linux") {
        vec!["xdg-open", "eog", "feh", "gwenview", "ristretto"]
    } else if cfg!(target_os = "macos") {
        vec!["open", "Preview"]
    } else if cfg!(target_os = "windows") {
        vec!["start", "mspaint"]
    } else {
        vec!["xdg-open"]
    };
    
    let mut success = false;
    for viewer in &viewers {
        let result = Command::new(viewer)
            .arg(&screenshot_path)
            .spawn();
            
        match result {
            Ok(mut child) => {
                // For some viewers (like xdg-open), we don't need to wait
                if viewer == &"xdg-open" || viewer == &"open" || viewer == &"start" {
                    success = true;
                    break;
                } else {
                    // For others, check if they started successfully
                    match child.try_wait()? {
                        Some(status) if status.success() => {
                            success = true;
                            break;
                        }
                        Some(_) => continue, // Failed, try next viewer
                        None => {
                            // Still running, consider it successful
                            success = true;
                            break;
                        }
                    }
                }
            }
            Err(_) => continue, // Try next viewer
        }
    }
    
    if success {
        println!("✅ Screenshot opened successfully");
    } else {
        println!("❌ Could not find a suitable image viewer");
        println!("Please install one of: {:?}", viewers);
        println!("Or open manually: {:?}", screenshot_path);
    }
    
    Ok(())
}