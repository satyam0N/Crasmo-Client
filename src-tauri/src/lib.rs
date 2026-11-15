pub mod auth;
pub mod download;
pub mod instances;
pub mod launch;
pub mod mods;
pub mod settings;
pub mod skins;
pub mod versions;

#[tauri::command]
fn ping() -> &'static str { "Crasmo Client backend ready" }
#[tauri::command]
fn list_instances() -> Result<Vec<serde_json::Value>, String> { Ok(Vec::new()) }
#[tauri::command]
fn get_settings() -> Result<serde_json::Value, String> { Ok(serde_json::json!({"memory_min_gb":2,"memory_max_gb":16,"memory_recommended_gb":6})) }
#[tauri::command]
fn save_settings(settings: serde_json::Value) -> Result<(), String> { if !settings.is_object(){return Err("Settings must be a JSON object".into())} Ok(()) }
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().invoke_handler(tauri::generate_handler![ping,list_instances,get_settings,save_settings]).run(tauri::generate_context!()).expect("error while running Crasmo Client"); }
