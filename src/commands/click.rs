use anyhow::Result;
use crate::config::Config;
use crate::grid::GridGenerator;
use crate::automation::MouseController;
use crate::session::SessionManager;

pub fn execute(
    config: &Config,
    zoom_square_id: u32,
    delay_override: Option<u64>,
    double_click: bool,
) -> Result<()> {
    // Load session data
    let session_data = SessionManager::load_session()?
        .ok_or_else(|| anyhow::anyhow!("No zoom session found. Run 'gui-helper zoom' first."))?;
    
    // Check if we have zoom data
    if session_data.zoom_area.is_none() || session_data.zoom_grid.is_empty() {
        return Err(anyhow::anyhow!("No zoom grid found. Run 'gui-helper zoom --square <ID>' first."));
    }
    
    // Find the zoom square
    let zoom_square = GridGenerator::find_zoom_square_by_id(&session_data.zoom_grid, zoom_square_id)
        .ok_or_else(|| anyhow::anyhow!("Zoom square '{}' not found. Available: 1-{}", 
                                       zoom_square_id, session_data.zoom_grid.len()))?;
    
    println!("Clicking zoom square {} at ({}, {})", 
             zoom_square.id, zoom_square.center_x, zoom_square.center_y);
    
    // Use override or config values
    let delay = delay_override.unwrap_or(config.automation.click_delay);
    let should_double_click = double_click || config.automation.double_click;
    
    // Create mouse controller and perform click
    let mut mouse = MouseController::new()?;
    
    println!("Performing {} click in {} ms...", 
             if should_double_click { "double" } else { "single" }, delay);
    
    mouse.click(
        zoom_square.center_x,
        zoom_square.center_y,
        delay,
        should_double_click,
    )?;
    
    println!("Click executed successfully at pixel coordinates ({}, {})", 
             zoom_square.center_x, zoom_square.center_y);
    
    // Show some context about the click
    if let Some(zoom_area) = &session_data.zoom_area {
        println!("Context: Clicked in zoom area of square '{}' at local position ({}, {})", 
                 zoom_area.parent_square, zoom_square.local_x, zoom_square.local_y);
    }
    
    Ok(())
}