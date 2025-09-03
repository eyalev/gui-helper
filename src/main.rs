mod config;
mod commands;
mod grid;
mod overlay;
mod screenshot;
mod automation;
mod session;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::Config;

#[derive(Parser)]
#[command(name = "gui-helper")]
#[command(about = "GUI automation helper with hierarchical grid overlay for AI agents")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Overview {
        #[arg(long, help = "Override config overview grid size")]
        grid_size: Option<u32>,
        #[arg(long, help = "How long to show overlay (seconds)")]
        duration: Option<u32>,
        #[arg(long, help = "Screenshot filename")]
        output: Option<String>,
        #[arg(long, help = "Export grid coordinates to JSON")]
        json: bool,
    },
    Zoom {
        #[arg(long, help = "Required: square ID from overview (e.g., 'B5')")]
        square: String,
        #[arg(long, help = "Override config padding")]
        padding: Option<u32>,
        #[arg(long, help = "Override config subdivision (NxN)")]
        subdivision: Option<u32>,
        #[arg(long, help = "How long to show zoom overlay (seconds)")]
        duration: Option<u32>,
        #[arg(long, help = "Zoom screenshot filename")]
        output: Option<String>,
        #[arg(long, help = "Export zoom grid coordinates")]
        json: bool,
    },
    Click {
        #[arg(long, help = "Required: zoom grid square number")]
        zoom: u32,
        #[arg(long, help = "Click delay override (ms)")]
        delay: Option<u64>,
        #[arg(long, help = "Perform double-click")]
        double: bool,
    },
    Config {
        #[arg(long, help = "Show current config")]
        show: bool,
        #[arg(long, help = "Reset to defaults")]
        reset: bool,
    },
    Session {
        #[arg(long, help = "Clear session data")]
        clear: bool,
    },
    Open {
        #[arg(long, help = "Open latest screenshot")]
        latest: bool,
        #[arg(help = "Specific screenshot path to open")]
        path: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    
    match cli.command {
        Commands::Overview { grid_size, duration, output, json } => {
            commands::overview::execute(&config, grid_size, duration, output, json)
        },
        Commands::Zoom { square, padding, subdivision, duration, output, json } => {
            commands::zoom::execute(&config, &square, padding, subdivision, duration, output, json)
        },
        Commands::Click { zoom, delay, double } => {
            commands::click::execute(&config, zoom, delay, double)
        },
        Commands::Config { show, reset } => {
            commands::config::execute(&config, show, reset)
        },
        Commands::Session { clear } => {
            commands::session::execute(clear)
        },
        Commands::Open { latest, path } => {
            commands::open::execute(latest, path)
        },
    }
}
