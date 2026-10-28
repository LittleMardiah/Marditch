use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub window_x: Option<i32>,
    pub window_y: Option<i32>,
    pub is_pinned: bool,
    pub first_run: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { window_x: None, window_y: None, is_pinned: false, first_run: true }
    }
}

pub fn get_config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = app_handle.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    fs::create_dir_all(&path).unwrap_or_default();
    path.push("config.json");
    path
}

pub fn load_config(app_handle: &tauri::AppHandle) -> AppConfig {
    let path = get_config_path(app_handle);
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    AppConfig::default()
}

pub fn save_config(app_handle: &tauri::AppHandle, config: &AppConfig) {
    let path = get_config_path(app_handle);
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, json);
    }
}
