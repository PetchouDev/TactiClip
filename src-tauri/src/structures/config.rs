use std::{path::PathBuf, sync::RwLock};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use once_cell::sync::Lazy;


// Global singleton configuration object
static CONFIG: Lazy<RwLock<Option<AppConfig>>> = Lazy::new(|| RwLock::new(None));

// Struct to represent the configuration of the application
#[derive(Serialize, Clone, Debug, Deserialize)]
#[serde(default = "AppConfig::default")]
pub struct AppConfig {
    // Window-specific fields
    pub window_position: String, // Changed from &'a str to String
    pub window_primary_factor: f64, // Width for top and bottom, height for left and right
    pub window_secondary_size: i32, // Height for top and bottom, width for left and right
    pub window_padding_x: i32, // Padding around the window on the x-axis
    pub window_padding_y: i32, // Padding around the window on the y-axis
    
    // Animation-specific fields 
    pub window_animation_duration: u64, // Duration of the animation in milliseconds
    pub window_steps: u64, // Number of steps for the animation
    pub ease_factor: i32, // Factor to ease the animation

    // Clipboard-specific fields
    pub window_rewrite_history_on_copy: bool, // Flag to indicate if the history should be rewritten on copy
    pub auto_hide_on_copy: bool, // Flag to indicate if the window should auto-hide on copy
    pub auto_paste_on_copy: bool, // Flag to indicate if the clipboard should be auto-pasted on copy
    pub max_displayed_characters: i32, // Maximum number of characters to display in the window

    // Scroll-specific fields  
    pub reset_scroll_on_show: bool, // Flag to indicate if the scroll should be reset on show
    pub scroll_factor: f64, // Factor to scroll the window
    pub smooth_scroll: bool, // Flag to indicate if the scroll should be smooth
}

// Implement the Default trait for AppConfig
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            // Window-specific fields
            window_position: "bottom".to_string(),
            window_primary_factor: 0.98,
            window_secondary_size: 250,
            window_padding_x: 0,
            window_padding_y: 50,

            // Animation-specific fields
            window_animation_duration: 100, // Duration in milliseconds
            window_steps: 30,
            ease_factor: 1,

            // Clipboard-specific fields
            window_rewrite_history_on_copy: false,
            auto_hide_on_copy: true,
            auto_paste_on_copy: false,
            max_displayed_characters: 250,

            // Scroll-specific fields
            reset_scroll_on_show: true,
            scroll_factor: 1.0,
            smooth_scroll: false,

            // 
        }
    }
}

// Implement methods for AppConfig
impl AppConfig {
    // Function to load the configuration from a JSON file
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = std::fs::read_to_string(file_path)?;
        let config: AppConfig = serde_json::from_str(&config_data)?;

        // Write the config to file to ensure all fields are present
        let _ = config.save_to_file(file_path);

        Ok(config)
    }

    // Function ot update the configuration from a JSON string
    pub fn update_from_json(&mut self, json_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let new_config: AppConfig = serde_json::from_str(json_str)?;
        *self = new_config;

        // Push the new singleton config to the global static variable
        let mut config_lock = CONFIG.write().unwrap();
        *config_lock = Some(self.clone());
        Ok(())
    }

    // Function to save the configuration to a JSON file
    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_data = serde_json::to_string_pretty(self)?;
        std::fs::write(file_path, config_data)?;
        Ok(())
    }
}


// Function to get the configuration path based on the operating system
pub fn get_config_path() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("Unable to access directories");

    // Détermine l'emplacement basé sur le système d'exploitation
    if cfg!(target_os = "windows") {
        // Sur Windows, utiliser AppData
        base_dirs
            .data_local_dir()
            .join("PetchouSoftware")
            .join("TactiClip")
            .join("config.json")
    } else {
        // Sur macOS et Linux, utiliser .local
        base_dirs
            .home_dir()
            .join(".local")
            .join("PetchouSoftware")
            .join("TactiClip")
            .join("config.json")
    }
}

// Function to get the configuration, loading it from a file if necessary
pub fn config() -> AppConfig {
  let mut config_lock = CONFIG.write().unwrap();
  if let Some(ref config) = *config_lock {
      // If config is already loaded, return it
      return config.clone();
  }

  // Path to the config file
  let config_path = get_config_path(); // Store in a variable

  // Check if config file exists, if not create it with default values
  if !config_path.exists() {
      let default_config = AppConfig::default();
      default_config.save_to_file(config_path.to_str().unwrap()).unwrap();
  }

  // Load config from the file
  let loaded_config = AppConfig::load_from_file(config_path.to_str().unwrap()).unwrap();
  
  // Save the loaded config to the global static variable
  *config_lock = Some(loaded_config.clone());

  loaded_config
}