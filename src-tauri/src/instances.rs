use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Loader { Vanilla, Fabric, Forge, Quilt }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub version: String,
    pub loader: Loader,
    pub game_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct NewInstance {
    pub name: String,
    pub version: String,
    pub loader: Loader,
}

#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum InstanceError {
    Io(String), InvalidInput(String), NotFound(String),
}

fn data_dir(app: &AppHandle) -> Result<PathBuf, InstanceError> {
    let dir = app.path().app_data_dir().map_err(|e| InstanceError::Io(e.to_string()))?;
    fs::create_dir_all(&dir).map_err(|e| InstanceError::Io(e.to_string()))?;
    Ok(dir)
}
fn instances_path(app: &AppHandle) -> Result<PathBuf, InstanceError> { Ok(data_dir(app)?.join("instances.json")) }
fn read_all(app: &AppHandle) -> Result<Vec<Instance>, InstanceError> {
    let path = instances_path(app)?;
    if !path.exists() { return Ok(Vec::new()); }
    let bytes = fs::read(path).map_err(|e| InstanceError::Io(e.to_string()))?;
    serde_json::from_slice(&bytes).map_err(|e| InstanceError::Io(format!("Instance data is invalid: {e}")))
}
fn write_all(app: &AppHandle, instances: &[Instance]) -> Result<(), InstanceError> {
    let path = instances_path(app)?;
    let temp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(instances).map_err(|e| InstanceError::Io(e.to_string()))?;
    fs::write(&temp, bytes).map_err(|e| InstanceError::Io(e.to_string()))?;
    fs::rename(temp, path).map_err(|e| InstanceError::Io(e.to_string()))
}
fn valid_component(value: &str) -> bool {
    !value.is_empty() && value.len() <= 64 && value != "." && value != ".."
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
}

#[tauri::command]
pub fn list_instances(app: AppHandle) -> Result<Vec<Instance>, InstanceError> { read_all(&app) }

#[tauri::command]
pub fn create_instance(app: AppHandle, input: NewInstance) -> Result<Instance, InstanceError> {
    if !valid_component(input.name.trim()) { return Err(InstanceError::InvalidInput("Name must be 1–64 letters, numbers, spaces, hyphens, or underscores".into())); }
    if input.version.trim().is_empty() || input.version.len() > 32 || !input.version.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_') {
        return Err(InstanceError::InvalidInput("Invalid game version".into()));
    }
    let mut instances = read_all(&app)?;
    let name = input.name.trim().to_owned();
    if instances.iter().any(|i| i.name.eq_ignore_ascii_case(&name)) { return Err(InstanceError::InvalidInput("An instance with that name already exists".into())); }
    let id = format!("{}-{:x}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos());
    let root = data_dir(&app)?.join("instances").join(&id);
    fs::create_dir_all(&root).map_err(|e| InstanceError::Io(e.to_string()))?;
    let instance = Instance { id, name, version: input.version, loader: input.loader, game_dir: root.to_string_lossy().into_owned() };
    instances.push(instance.clone());
    if let Err(error) = write_all(&app, &instances) { let _ = fs::remove_dir_all(&root); return Err(error); }
    Ok(instance)
}

#[tauri::command]
pub fn delete_instance(app: AppHandle, id: String) -> Result<(), InstanceError> {
    let mut instances = read_all(&app)?;
    let index = instances.iter().position(|instance| instance.id == id).ok_or_else(|| InstanceError::NotFound("Instance not found".into()))?;
    let removed = instances.remove(index);
    write_all(&app, &instances)?;
    // Deleting metadata must not recursively remove user worlds without an explicit confirmation flow.
    let _ = removed;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::valid_component;
    #[test] fn rejects_path_traversal_and_unsafe_names() {
        assert!(valid_component("My Survival World"));
        assert!(!valid_component("../outside"));
        assert!(!valid_component(""));
        assert!(!valid_component("name/other"));
    }
}
