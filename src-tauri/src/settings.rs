use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub memory_min_gb: u8,
    pub memory_max_gb: u8,
    pub memory_recommended_gb: u8,
    pub close_on_launch: bool,
    pub show_snapshots: bool,
    pub game_directory: Option<String>,
}
impl Default for Settings {
    fn default() -> Self { Self { memory_min_gb: 2, memory_max_gb: 16, memory_recommended_gb: 6, close_on_launch: false, show_snapshots: false, game_directory: None } }
}
impl Settings {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.memory_min_gb < 1 || self.memory_max_gb > 128 || self.memory_min_gb > self.memory_max_gb
            || !(self.memory_min_gb..=self.memory_max_gb).contains(&self.memory_recommended_gb) {
            return Err("Invalid memory configuration");
        }
        if let Some(path) = &self.game_directory {
            if path.trim().is_empty() || path.len() > 4096 { return Err("Invalid game directory"); }
        }
        Ok(())
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum SettingsError { Io(String), InvalidInput(String) }
fn path(app: &AppHandle) -> Result<PathBuf, SettingsError> {
    let dir = app.path().app_data_dir().map_err(|e| SettingsError::Io(e.to_string()))?;
    fs::create_dir_all(&dir).map_err(|e| SettingsError::Io(e.to_string()))?;
    Ok(dir.join("settings.json"))
}
#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, SettingsError> {
    let path = path(&app)?;
    if !path.exists() { return Ok(Settings::default()); }
    let data = fs::read(path).map_err(|e| SettingsError::Io(e.to_string()))?;
    let value: Settings = serde_json::from_slice(&data).map_err(|e| SettingsError::Io(format!("Settings file is invalid: {e}")))?;
    value.validate().map_err(|e| SettingsError::InvalidInput(e.into()))?;
    Ok(value)
}
#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), SettingsError> {
    settings.validate().map_err(|e| SettingsError::InvalidInput(e.into()))?;
    let path = path(&app)?;
    let temp = path.with_extension("json.tmp");
    let data = serde_json::to_vec_pretty(&settings).map_err(|e| SettingsError::Io(e.to_string()))?;
    fs::write(&temp, data).map_err(|e| SettingsError::Io(e.to_string()))?;
    fs::rename(temp, path).map_err(|e| SettingsError::Io(e.to_string()))
}
#[cfg(test)]
mod tests {
    use super::Settings;
    #[test] fn accepts_default_and_rejects_out_of_range_memory() {
        assert!(Settings::default().validate().is_ok());
        let invalid = Settings { memory_min_gb: 10, memory_max_gb: 4, ..Settings::default() };
        assert!(invalid.validate().is_err());
    }
}
