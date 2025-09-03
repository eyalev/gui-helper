use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config, show: bool, reset: bool) -> Result<()> {
    if reset {
        let default_config = Config::default();
        default_config.save()?;
        println!("Configuration reset to defaults");
        return Ok(());
    }
    
    if show {
        let config_toml = toml::to_string_pretty(config)?;
        println!("Current configuration:");
        println!("=====================");
        println!("{}", config_toml);
        
        let config_path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("gui-helper")
            .join("config.toml");
            
        println!("\nConfiguration file location: {:?}", config_path);
        return Ok(());
    }
    
    // If neither show nor reset, display help
    println!("Config command options:");
    println!("  --show   Show current configuration");
    println!("  --reset  Reset configuration to defaults");
    
    Ok(())
}