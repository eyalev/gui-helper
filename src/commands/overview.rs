use anyhow::Result;
use crate::config::Config;
use crate::grid::{GridGenerator, SessionData};
use crate::overlay::SimpleOverlay;
use crate::screenshot::ScreenshotCapture;
use crate::session::SessionManager;
use chrono::Utc;
use std::path::PathBuf;

pub fn execute(
    config: &Config,
    grid_size_override: Option<u32>,
    duration_override: Option<u32>,
    output_override: Option<String>,
    export_json: bool,
) -> Result<()> {
    // Get screen dimensions
    let (screen_width, screen_height) = ScreenshotCapture::get_primary_display_size()?;
    
    // Use override or config values
    let grid_size = grid_size_override.unwrap_or(config.overview_grid.square_size);
    let duration = duration_override.unwrap_or(config.display.duration);
    
    // Generate grid
    let (overview_squares, screen_info) = GridGenerator::generate_overview_grid(
        screen_width, 
        screen_height, 
        grid_size
    );
    
    println!("Generated overview grid: {}x{} squares ({}x{} pixels)", 
             screen_info.cols, screen_info.rows, screen_width, screen_height);
    
    // Show overlay with grid (simplified version)
    SimpleOverlay::show_overview_grid(duration)?;
    
    println!("Overlay displayed for {} seconds", duration);
    
    // Take screenshot
    let screenshots_dir = SessionManager::get_screenshots_dir()?;
    let filename = if let Some(custom_name) = output_override {
        custom_name
    } else {
        SessionManager::generate_screenshot_filename("overview", &config.display.output_format)
    };
    
    let output_path = screenshots_dir.join(&filename);
    let (_captured_width, _captured_height) = ScreenshotCapture::capture_and_save(
        &output_path, 
        &config.display.output_format
    )?;
    
    println!("Screenshot saved: {:?}", output_path);
    
    // Create session data
    let session_data = SessionData {
        overview_grid: overview_squares.clone(),
        selected_square: None,
        zoom_area: None,
        zoom_grid: vec![],
        screen_info,
        timestamp: Utc::now(),
    };
    
    // Save session
    SessionManager::save_session(&session_data)?;
    
    if export_json {
        let json_filename = PathBuf::from(&filename).with_extension("json");
        let json_path = screenshots_dir.join(json_filename);
        let json_data = serde_json::to_string_pretty(&session_data)?;
        std::fs::write(&json_path, json_data)?;
        println!("Grid data exported: {:?}", json_path);
    }
    
    println!("Overview command completed successfully!");
    println!("Use 'gui-helper zoom --square <ID>' to zoom into a specific square");
    println!("Available squares: A1-{}{}", 
             char::from(b'A' + (session_data.screen_info.rows - 1) as u8),
             session_data.screen_info.cols);
    
    Ok(())
}