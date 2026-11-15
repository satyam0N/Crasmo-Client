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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ping,
            instances::list_instances,
            instances::create_instance,
            instances::delete_instance,
            settings::get_settings,
            settings::save_settings,
            versions::get_version_manifest
        ])
        .run(tauri::generate_context!())
        .expect("error while running Crasmo Client");
}
