use anyhow::Result;
use crate::grid::SessionData;
use crate::config::Config;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;

pub struct SessionManager;

impl SessionManager {
    pub fn save_session(session_data: &SessionData) -> Result<()> {
        let session_path = Self::session_path()?;
        
        if let Some(parent) = session_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json_data = serde_json::to_string_pretty(session_data)?;
        fs::write(&session_path, json_data)?;
        Ok(())
    }
    
    pub fn load_session() -> Result<Option<SessionData>> {
        let session_path = Self::session_path()?;
        
        if !session_path.exists() {
            return Ok(None);
        }
        
        let contents = fs::read_to_string(&session_path)?;
        let session_data: SessionData = serde_json::from_str(&contents)?;
        
        // Check if session is recent (within last hour)
        let now = Utc::now();
        let session_age = now.signed_duration_since(session_data.timestamp);
        
        if session_age.num_minutes() > 60 {
            // Session is too old, clear it
            Self::clear_session()?;
            return Ok(None);
        }
        
        Ok(Some(session_data))
    }
    
    pub fn clear_session() -> Result<()> {
        let session_path = Self::session_path()?;
        
        if session_path.exists() {
            fs::remove_file(&session_path)?;
        }
        
        Ok(())
    }
    
    pub fn update_session_timestamp(mut session_data: SessionData) -> Result<SessionData> {
        session_data.timestamp = Utc::now();
        Self::save_session(&session_data)?;
        Ok(session_data)
    }
    
    fn session_path() -> Result<PathBuf> {
        let data_dir = Config::data_dir()?;
        Ok(data_dir.join("session.json"))
    }
    
    pub fn get_screenshots_dir() -> Result<PathBuf> {
        let data_dir = Config::data_dir()?;
        let screenshots_dir = data_dir.join("screenshots");
        
        if !screenshots_dir.exists() {
            fs::create_dir_all(&screenshots_dir)?;
        }
        
        Ok(screenshots_dir)
    }
    
    pub fn generate_screenshot_filename(prefix: &str, format: &str) -> String {
        let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        format!("{}_{}.{}", prefix, timestamp, format)
    }
    
    pub fn get_latest_screenshot() -> Result<Option<PathBuf>> {
        let screenshots_dir = Self::get_screenshots_dir()?;
        
        if !screenshots_dir.exists() {
            return Ok(None);
        }
        
        let mut entries: Vec<_> = fs::read_dir(&screenshots_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg"))
                    .unwrap_or(false)
            })
            .collect();
        
        // Sort by modification time (newest first)
        entries.sort_by(|a, b| {
            let time_a = a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
            let time_b = b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
            time_b.cmp(&time_a)
        });
        
        Ok(entries.first().map(|entry| entry.path()))
    }
}