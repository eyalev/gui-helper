use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub overview_grid: OverviewGridConfig,
    pub zoom_grid: ZoomGridConfig,
    pub display: DisplayConfig,
    pub automation: AutomationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewGridConfig {
    pub square_size: u32,
    pub color: String,
    pub thickness: u32,
    pub opacity: f32,
    pub show_numbers: bool,
    pub font_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomGridConfig {
    pub padding: u32,
    pub subdivision: u32,
    pub color: String,
    pub thickness: u32,
    pub opacity: f32,
    pub show_numbers: bool,
    pub font_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub duration: u32,
    pub output_format: String,
    pub save_screenshots: bool,
    pub screenshot_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    pub click_delay: u64,
    pub double_click: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            overview_grid: OverviewGridConfig {
                square_size: 100,
                color: "red".to_string(),
                thickness: 2,
                opacity: 0.7,
                show_numbers: true,
                font_size: 16,
            },
            zoom_grid: ZoomGridConfig {
                padding: 50,
                subdivision: 10,
                color: "blue".to_string(),
                thickness: 1,
                opacity: 0.8,
                show_numbers: true,
                font_size: 12,
            },
            display: DisplayConfig {
                duration: 3,
                output_format: "png".to_string(),
                save_screenshots: true,
                screenshot_dir: "./screenshots".to_string(),
            },
            automation: AutomationConfig {
                click_delay: 100,
                double_click: false,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = toml::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("gui-helper").join("config.toml"))
    }
    
    pub fn data_dir() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?;
        Ok(data_dir.join("gui-helper"))
    }
}