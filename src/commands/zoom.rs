use anyhow::Result;
use crate::config::Config;
use crate::grid::GridGenerator;
use crate::overlay::SimpleOverlay;
use crate::screenshot::ScreenshotCapture;
use crate::session::SessionManager;
use std::path::PathBuf;

pub fn execute(
    config: &Config,
    square_id: &str,
    padding_override: Option<u32>,
    subdivision_override: Option<u32>,
    duration_override: Option<u32>,
    output_override: Option<String>,
    export_json: bool,
) -> Result<()> {
    // Load session data
    let mut session_data = SessionManager::load_session()?
        .ok_or_else(|| anyhow::anyhow!("No overview session found. Run 'gui-helper overview' first."))?;
    
    // Find the selected square
    let parent_square = GridGenerator::find_square_by_id(&session_data.overview_grid, square_id)
        .ok_or_else(|| anyhow::anyhow!("Square '{}' not found in overview grid", square_id))?
        .clone();
    
    println!("Zooming into square '{}' at ({}, {})", 
             parent_square.id, parent_square.x, parent_square.y);
    
    // Use override or config values
    let padding = padding_override.unwrap_or(config.zoom_grid.padding);
    let subdivision = subdivision_override.unwrap_or(config.zoom_grid.subdivision);
    let duration = duration_override.unwrap_or(config.display.duration);
    
    // Generate zoom grid
    let (zoom_area, zoom_squares) = GridGenerator::generate_zoom_grid(&parent_square, padding, subdivision);
    
    println!("Generated zoom grid: {}x{} squares in area {}x{}", 
             subdivision, subdivision, zoom_area.width, zoom_area.height);
    
    // Show overlay with zoom grid (simplified version)
    let zoom_info = format!("Zooming area {}x{} at ({}, {})", 
                           zoom_area.width, zoom_area.height, zoom_area.x, zoom_area.y);
    SimpleOverlay::show_zoom_grid(&zoom_info, duration)?;
    
    println!("Zoom overlay displayed for {} seconds", duration);
    
    // Take screenshot of zoom area
    let screenshots_dir = SessionManager::get_screenshots_dir()?;
    let filename = if let Some(custom_name) = output_override {
        custom_name
    } else {
        SessionManager::generate_screenshot_filename(
            &format!("zoom_{}", square_id), 
            &config.display.output_format
        )
    };
    
    let output_path = screenshots_dir.join(&filename);
    ScreenshotCapture::capture_area_and_save(
        zoom_area.x,
        zoom_area.y,
        zoom_area.width,
        zoom_area.height,
        &output_path,
        &config.display.output_format,
    )?;
    
    println!("Zoom screenshot saved: {:?}", output_path);
    
    // Update session data
    session_data.selected_square = Some(square_id.to_string());
    session_data.zoom_area = Some(zoom_area);
    session_data.zoom_grid = zoom_squares.clone();
    session_data = SessionManager::update_session_timestamp(session_data)?;
    
    if export_json {
        let json_filename = PathBuf::from(&filename).with_extension("json");
        let json_path = screenshots_dir.join(json_filename);
        let json_data = serde_json::to_string_pretty(&session_data)?;
        std::fs::write(&json_path, json_data)?;
        println!("Zoom grid data exported: {:?}", json_path);
    }
    
    println!("Zoom command completed successfully!");
    println!("Use 'gui-helper click --zoom <NUMBER>' to click on a zoom square");
    println!("Available zoom squares: 1-{}", zoom_squares.len());
    
    Ok(())
}