use anyhow::Result;
use std::process::Command;

pub struct WindowController;

impl WindowController {
    pub fn maximize_active_window() -> Result<()> {
        // Use wmctrl to properly maximize the active window with window manager state
        let maximize_output = Command::new("wmctrl")
            .args(&["-r", ":ACTIVE:", "-b", "add,maximized_vert,maximized_horz"])
            .output()?;
        
        if !maximize_output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to maximize window: {}",
                String::from_utf8_lossy(&maximize_output.stderr)
            ));
        }
        
        Ok(())
    }
    
    pub fn maximize_window_by_name(window_name: &str) -> Result<()> {
        // Use wmctrl to maximize a window by name
        let maximize_output = Command::new("wmctrl")
            .args(&["-r", window_name, "-b", "add,maximized_vert,maximized_horz"])
            .output()?;
        
        if !maximize_output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to maximize window '{}': {}",
                window_name,
                String::from_utf8_lossy(&maximize_output.stderr)
            ));
        }
        
        Ok(())
    }
    
    pub fn restore_window() -> Result<()> {
        // Use wmctrl to restore (un-maximize) the currently active window
        let restore_output = Command::new("wmctrl")
            .args(&["-r", ":ACTIVE:", "-b", "remove,maximized_vert,maximized_horz"])
            .output()?;
        
        if !restore_output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to restore window: {}",
                String::from_utf8_lossy(&restore_output.stderr)
            ));
        }
        
        Ok(())
    }
    
    pub fn list_windows() -> Result<Vec<(String, String)>> {
        // Get list of windows with ID and title
        let output = Command::new("wmctrl")
            .args(&["-l"])
            .output()?;
        
        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to list windows: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        let window_list = String::from_utf8(output.stdout)?;
        let mut windows = Vec::new();
        
        for line in window_list.lines() {
            if let Some(title_start) = line.rfind(' ') {
                if title_start > 0 {
                    let parts: Vec<&str> = line.splitn(4, ' ').collect();
                    if parts.len() >= 4 {
                        let id = parts[0].to_string();
                        let title = parts[3].to_string();
                        // Skip desktop windows
                        if !title.starts_with('@') {
                            windows.push((id, title));
                        }
                    }
                }
            }
        }
        
        Ok(windows)
    }
    
    pub fn focus_window(window_name: &str) -> Result<()> {
        // First try exact wmctrl matching
        let focus_output = Command::new("wmctrl")
            .args(&["-a", window_name])
            .output()?;
        
        if focus_output.status.success() {
            return Ok(());
        }
        
        // If exact match fails, try to find a matching window
        let windows = Self::list_windows()?;
        let mut matches = Vec::new();
        
        let search_words: Vec<String> = window_name.to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        for (id, title) in windows {
            let title_lower = title.to_lowercase();
            
            // Check if all search words are present in the title
            let all_words_match = search_words.iter()
                .all(|word| title_lower.contains(word));
            
            
            if all_words_match {
                matches.push((id, title));
            }
        }
        
        // If no exact matches, try fuzzy matching as fallback
        if matches.is_empty() {
            let mut fuzzy_matches = Vec::new();
            
            for (id, title) in Self::list_windows()? {
                let title_lower = title.to_lowercase();
                let mut score = 0;
                let mut exact_matches = 0;
                
                for search_word in &search_words {
                    // Check for exact matches first (higher score)
                    if title_lower.contains(search_word) {
                        score += 10; // Higher score for exact matches
                        exact_matches += 1;
                    } else if Self::fuzzy_match(search_word, &title_lower) {
                        score += 1; // Lower score for fuzzy matches
                    }
                }
                
                
                // If at least half the words match (exact or fuzzy), consider it a match
                let total_matches = exact_matches + (score - exact_matches * 10);
                if total_matches >= (search_words.len() + 1) / 2 {
                    fuzzy_matches.push((id, title, score));
                }
            }
            
            // Sort by score (highest first)
            fuzzy_matches.sort_by(|a, b| b.2.cmp(&a.2));
            
            if !fuzzy_matches.is_empty() {
                // If there's a clear winner (significantly higher score), pick it automatically
                if fuzzy_matches.len() > 1 {
                    let best_score = fuzzy_matches[0].2;
                    let second_best_score = fuzzy_matches[1].2;
                    
                    // If best score is higher or equal with at least one exact match, use only the best match
                    if best_score > second_best_score {
                        matches = vec![(fuzzy_matches[0].0.clone(), fuzzy_matches[0].1.clone())];
                    } else {
                        matches = fuzzy_matches.into_iter().map(|(id, title, _)| (id, title)).collect();
                    }
                } else {
                    matches = fuzzy_matches.into_iter().map(|(id, title, _)| (id, title)).collect();
                }
            }
        }

        match matches.len() {
            0 => Err(anyhow::anyhow!(
                "No window found matching '{}'. Use --list to see available windows.",
                window_name
            )),
            1 => {
                // Found exactly one match, focus it by ID
                let (window_id, _) = &matches[0];
                let focus_output = Command::new("wmctrl")
                    .args(&["-i", "-a", window_id])
                    .output()?;
                
                if !focus_output.status.success() {
                    return Err(anyhow::anyhow!(
                        "Failed to focus window: {}",
                        String::from_utf8_lossy(&focus_output.stderr)
                    ));
                }
                
                Ok(())
            }
            _ => {
                // Multiple matches, show them to user
                let mut error_msg = format!("Multiple windows match '{}'. Please be more specific:\n", window_name);
                for (_, title) in matches {
                    error_msg.push_str(&format!("  - {}\n", title));
                }
                Err(anyhow::anyhow!("{}", error_msg.trim()))
            }
        }
    }
    
    pub fn focus_and_maximize_window(window_name: &str) -> Result<()> {
        // First focus the window, then maximize it
        Self::focus_window(window_name)?;
        
        // Small delay to ensure window is focused before maximizing
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Now maximize the focused window
        Self::maximize_active_window()
    }
    
    pub fn focus_and_unmaximize_window(window_name: &str) -> Result<()> {
        // First focus the window, then restore it
        Self::focus_window(window_name)?;
        
        // Small delay to ensure window is focused before restoring
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Now restore the focused window
        Self::restore_window()
    }
    
    fn fuzzy_match(search_word: &str, text: &str) -> bool {
        // Simple fuzzy matching strategies
        
        // 1. Direct substring match
        if text.contains(search_word) {
            return true;
        }
        
        // 2. Handle common abbreviations/expansions
        let expansions = match search_word {
            "vscode" => vec!["visual studio code", "vs code"],
            "chrome" => vec!["google chrome"],
            "firefox" => vec!["mozilla firefox"],
            _ => vec![],
        };
        
        for expansion in expansions {
            if text.contains(expansion) {
                return true;
            }
        }
        
        // 3. Check if search word appears as separate letters (initials)
        // For example "vsc" should match "Visual Studio Code"
        if search_word.len() >= 2 {
            let words: Vec<&str> = text.split_whitespace().collect();
            let initials: String = words.iter()
                .filter_map(|word| word.chars().next())
                .collect::<String>()
                .to_lowercase();
            
            if initials.contains(search_word) {
                return true;
            }
        }
        
        // 4. Levenshtein-like partial match for similar words
        if search_word.len() >= 3 && Self::partial_similarity(search_word, text) {
            return true;
        }
        
        false
    }
    
    fn partial_similarity(search_word: &str, text: &str) -> bool {
        // Simple similarity check - look for words in text that share most characters
        let search_chars: std::collections::HashSet<char> = search_word.chars().collect();
        
        for word in text.split_whitespace() {
            if word.len() >= search_word.len() {
                let word_chars: std::collections::HashSet<char> = word.chars().collect();
                let common_chars = search_chars.intersection(&word_chars).count();
                
                // If more than 70% of characters match, consider it similar
                if common_chars as f64 / search_word.len() as f64 > 0.7 {
                    return true;
                }
            }
        }
        
        false
    }
}