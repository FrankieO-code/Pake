use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use std::env;
use tauri::command;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppData {
    pub devices: Vec<serde_json::Value>,
    pub bookings: Vec<serde_json::Value>,
    pub nextDeviceId: i32,
    pub nextBookingId: i32,
}

fn get_data_file_path() -> PathBuf {
    let exe_path = env::current_exe().expect("无法获取可执行文件路径");
    let exe_dir = exe_path.parent().expect("无法获取可执行文件所在目录");
    exe_dir.join("data.json")
}

#[command]
pub async fn load_app_data() -> Result<Option<AppData>, String> {
    let path = get_data_file_path();
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    let data: AppData = serde_json::from_str(&content)
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    Ok(Some(data))
}

#[command]
pub async fn save_app_data(data: AppData) -> Result<(), String> {
    let path = get_data_file_path();
    let json_str = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;
    fs::write(&path, json_str)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}
