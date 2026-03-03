use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub initial_minutes: u32,
    pub video_path: String,
    #[serde(default = "default_video_volume")]
    pub video_volume: f64,
    #[serde(default = "default_brown_noise_volume")]
    pub brown_noise_volume: f64,
}

fn default_video_volume() -> f64 {
    1.0
}

fn default_brown_noise_volume() -> f64 {
    0.6
}

impl UserConfig {
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(&home).join(".config/pomodoro-service/config.toml")
    }

    pub fn load() -> Self {
        let config_path = Self::config_path();
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());

        let mut config = if let Ok(content) = fs::read_to_string(config_path) {
            toml::from_str(&content).unwrap_or_else(|_| Self::default())
        } else {
            Self::default()
        };

        if config.video_path.starts_with("./") {
            config.video_path = config.video_path.replacen(".", &home, 1);
        }

        config
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config dir: {}", e))?;
        }

        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(config_path, toml_str).map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            initial_minutes: 25,
            video_path: "./relax.mp4".to_string(),
            video_volume: default_video_volume(),
            brown_noise_volume: default_brown_noise_volume(),
        }
    }
}
