use anyhow::Result;
use screenshots::Screen;
use image::{ImageBuffer, RgbaImage};
use std::path::Path;

pub struct ScreenshotCapture;

impl ScreenshotCapture {
    pub fn get_primary_display_size() -> Result<(u32, u32)> {
        let screens = Screen::all()?;
        if let Some(primary) = screens.first() {
            Ok((primary.display_info.width, primary.display_info.height))
        } else {
            Err(anyhow::anyhow!("No displays found"))
        }
    }
    
    pub fn capture_screen() -> Result<RgbaImage> {
        let screens = Screen::all()?;
        let primary = screens.first()
            .ok_or_else(|| anyhow::anyhow!("No displays found"))?;
            
        let image = primary.capture()?;
        
        // Convert from screenshots::Image to image::RgbaImage
        let width = image.width();
        let height = image.height();
        let rgba_data = image.rgba();
        
        ImageBuffer::from_raw(width, height, rgba_data.to_vec())
            .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))
    }
    
    pub fn capture_and_save(output_path: &Path, format: &str) -> Result<(u32, u32)> {
        let image = Self::capture_screen()?;
        let (width, height) = (image.width(), image.height());
        
        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        match format.to_lowercase().as_str() {
            "png" => image.save_with_format(output_path, image::ImageFormat::Png)?,
            "jpeg" | "jpg" => image.save_with_format(output_path, image::ImageFormat::Jpeg)?,
            _ => return Err(anyhow::anyhow!("Unsupported image format: {}", format)),
        }
        
        Ok((width, height))
    }
    
    pub fn capture_area(x: u32, y: u32, width: u32, height: u32) -> Result<RgbaImage> {
        let full_image = Self::capture_screen()?;
        
        // Crop the image to the specified area
        let cropped = image::imageops::crop_imm(&full_image, x, y, width, height);
        Ok(cropped.to_image())
    }
    
    pub fn capture_area_and_save(
        x: u32, 
        y: u32, 
        width: u32, 
        height: u32, 
        output_path: &Path, 
        format: &str
    ) -> Result<()> {
        let cropped_image = Self::capture_area(x, y, width, height)?;
        
        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        match format.to_lowercase().as_str() {
            "png" => cropped_image.save_with_format(output_path, image::ImageFormat::Png)?,
            "jpeg" | "jpg" => cropped_image.save_with_format(output_path, image::ImageFormat::Jpeg)?,
            _ => return Err(anyhow::anyhow!("Unsupported image format: {}", format)),
        }
        
        Ok(())
    }
}