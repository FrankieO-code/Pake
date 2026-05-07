use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppData {
    pub devices: Vec<serde_json::Value>,
    pub bookings: Vec<serde_json::Value>,
    pub nextDeviceId: i32,
    pub nextBookingId: i32,
}

fn get_data_file_path() -> PathBuf {
    let exe_path = env::current_exe().expect("failed to get exe path");
    let exe_dir = exe_path.parent().expect("failed to get exe directory");
    exe_dir.join("data.json")
}

#[command]
pub fn load_app_data() -> Result<Option<AppData>, String> {
    let path = get_data_file_path();
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("failed to read file: {}", e))?;
    let data: AppData = serde_json::from_str(&content).map_err(|e| format!("failed to parse JSON: {}", e))?;
    Ok(Some(data))
}

#[command]
pub fn save_app_data(data: AppData) -> Result<(), String> {
    let path = get_data_file_path();
    let json_str = serde_json::to_string_pretty(&data).map_err(|e| format!("failed to serialize JSON: {}", e))?;
    fs::write(&path, json_str).map_err(|e| format!("failed to write file: {}", e))?;
    Ok(())
}
