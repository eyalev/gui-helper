use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewSquare {
    pub id: String,        // "A1"
    pub numeric_id: usize, // 0-based index
    pub row: usize,        // 0-based row
    pub col: usize,        // 0-based column
    pub x: u32,            // Top-left x coordinate
    pub y: u32,            // Top-left y coordinate
    pub width: u32,        // Square width
    pub height: u32,       // Square height
    pub center_x: u32,     // Center point for clicking
    pub center_y: u32,     // Center point for clicking
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomSquare {
    pub id: u32,           // 1-based numeric ID (1, 2, 3...)
    pub row: usize,        // 0-based row within zoom area
    pub col: usize,        // 0-based column within zoom area
    pub local_x: u32,      // Position within zoom area
    pub local_y: u32,      // Position within zoom area
    pub abs_x: u32,        // Absolute screen coordinates
    pub abs_y: u32,        // Absolute screen coordinates
    pub width: u32,        // Square width
    pub height: u32,       // Square height
    pub center_x: u32,     // Absolute center point for clicking
    pub center_y: u32,     // Absolute center point for clicking
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomArea {
    pub parent_square: String, // Original overview square ID (e.g., "B5")
    pub x: u32,               // Top-left x of zoom area
    pub y: u32,               // Top-left y of zoom area
    pub width: u32,           // Width of zoom area (including padding)
    pub height: u32,          // Height of zoom area (including padding)
    pub subdivision: u32,     // Number of subdivisions per axis
    pub padding: u32,         // Padding around original square
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub grid_size: u32,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub overview_grid: Vec<OverviewSquare>,
    pub selected_square: Option<String>,
    pub zoom_area: Option<ZoomArea>,
    pub zoom_grid: Vec<ZoomSquare>,
    pub screen_info: ScreenInfo,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct GridGenerator;

impl GridGenerator {
    pub fn generate_overview_grid(screen_width: u32, screen_height: u32, grid_size: u32) -> (Vec<OverviewSquare>, ScreenInfo) {
        let cols = (screen_width / grid_size) as usize;
        let rows = (screen_height / grid_size) as usize;
        
        let mut squares = Vec::new();
        let mut numeric_id = 0;
        
        for row in 0..rows {
            for col in 0..cols {
                let x = col as u32 * grid_size;
                let y = row as u32 * grid_size;
                let id = Self::generate_alphanumeric_id(row, col);
                
                squares.push(OverviewSquare {
                    id,
                    numeric_id,
                    row,
                    col,
                    x,
                    y,
                    width: grid_size,
                    height: grid_size,
                    center_x: x + grid_size / 2,
                    center_y: y + grid_size / 2,
                });
                
                numeric_id += 1;
            }
        }
        
        let screen_info = ScreenInfo {
            width: screen_width,
            height: screen_height,
            grid_size,
            rows,
            cols,
        };
        
        (squares, screen_info)
    }
    
    pub fn generate_zoom_grid(parent_square: &OverviewSquare, padding: u32, subdivision: u32) -> (ZoomArea, Vec<ZoomSquare>) {
        let zoom_area = ZoomArea {
            parent_square: parent_square.id.clone(),
            x: parent_square.x.saturating_sub(padding),
            y: parent_square.y.saturating_sub(padding),
            width: parent_square.width + (padding * 2),
            height: parent_square.height + (padding * 2),
            subdivision,
            padding,
        };
        
        let sub_width = zoom_area.width / subdivision;
        let sub_height = zoom_area.height / subdivision;
        
        let mut squares = Vec::new();
        let mut id = 1;
        
        for row in 0..subdivision {
            for col in 0..subdivision {
                let local_x = col * sub_width;
                let local_y = row * sub_height;
                let abs_x = zoom_area.x + local_x;
                let abs_y = zoom_area.y + local_y;
                
                squares.push(ZoomSquare {
                    id,
                    row: row as usize,
                    col: col as usize,
                    local_x,
                    local_y,
                    abs_x,
                    abs_y,
                    width: sub_width,
                    height: sub_height,
                    center_x: abs_x + sub_width / 2,
                    center_y: abs_y + sub_height / 2,
                });
                
                id += 1;
            }
        }
        
        (zoom_area, squares)
    }
    
    fn generate_alphanumeric_id(row: usize, col: usize) -> String {
        let letter = char::from(b'A' + (row % 26) as u8);
        format!("{}{}", letter, col + 1)
    }
    
    pub fn find_square_by_id<'a>(squares: &'a [OverviewSquare], id: &str) -> Option<&'a OverviewSquare> {
        squares.iter().find(|s| s.id == id)
    }
    
    pub fn find_zoom_square_by_id<'a>(squares: &'a [ZoomSquare], id: u32) -> Option<&'a ZoomSquare> {
        squares.iter().find(|s| s.id == id)
    }
}