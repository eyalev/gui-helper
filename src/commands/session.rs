use anyhow::Result;
use crate::session::SessionManager;

pub fn execute(clear: bool) -> Result<()> {
    if clear {
        SessionManager::clear_session()?;
        println!("Session data cleared");
        return Ok(());
    }
    
    // Show session status
    match SessionManager::load_session()? {
        Some(session_data) => {
            println!("Active session found:");
            println!("  Screen: {}x{}", session_data.screen_info.width, session_data.screen_info.height);
            println!("  Grid: {}x{} squares", session_data.screen_info.cols, session_data.screen_info.rows);
            let local_time = session_data.timestamp.with_timezone(&chrono::Local);
            println!("  Timestamp: {}", local_time.format("%B %d, %Y at %l:%M %p"));
            
            if let Some(selected_square) = &session_data.selected_square {
                println!("  Selected square: {}", selected_square);
            }
            
            if let Some(zoom_area) = &session_data.zoom_area {
                println!("  Zoom area: {} ({}x{} with {} padding)", 
                         zoom_area.parent_square, 
                         zoom_area.width, 
                         zoom_area.height,
                         zoom_area.padding);
                println!("  Zoom squares: {}", session_data.zoom_grid.len());
            }
        }
        None => {
            println!("No active session found");
        }
    }
    
    Ok(())
}