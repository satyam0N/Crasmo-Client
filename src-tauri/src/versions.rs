use serde::Serialize;
use std::{fs, time::{Duration, SystemTime}};
use tauri::{AppHandle, Manager};

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum VersionError { Network(String), InvalidManifest(String), Io(String) }
#[derive(Debug, Clone, Serialize)]
pub struct VersionEntry { pub id: String, #[serde(rename = "type")] pub kind: String, pub url: String, pub release_time: String }
#[derive(Debug, Serialize)]
pub struct VersionManifest { pub latest: serde_json::Value, pub versions: Vec<VersionEntry> }
#[tauri::command]
pub async fn get_version_manifest(app: AppHandle, include_snapshots: bool) -> Result<VersionManifest, VersionError> {
    let dir = app.path().app_cache_dir().map_err(|e| VersionError::Io(e.to_string()))?;
    fs::create_dir_all(&dir).map_err(|e| VersionError::Io(e.to_string()))?;
    let cache = dir.join("mojang-version-manifest-v2.json");
    let cache_fresh = fs::metadata(&cache).and_then(|m| m.modified()).ok()
        .and_then(|time| SystemTime::now().duration_since(time).ok()).is_some_and(|age| age < Duration::from_secs(6 * 60 * 60));
    let raw = if cache_fresh {
        fs::read(&cache).map_err(|e| VersionError::Io(e.to_string()))?
    } else {
        match reqwest::get(MANIFEST_URL).await {
            Ok(response) if response.status().is_success() => {
                let bytes = response.bytes().await.map_err(|e| VersionError::Network(e.to_string()))?;
                fs::write(&cache, &bytes).map_err(|e| VersionError::Io(e.to_string()))?;
                bytes.to_vec()
            }
            Ok(response) => {
                if cache.exists() { fs::read(&cache).map_err(|e| VersionError::Io(e.to_string()))? }
                else { return Err(VersionError::Network(format!("Mojang manifest returned HTTP {}", response.status()))); }
            }
            Err(error) if cache.exists() => fs::read(&cache).map_err(|e| VersionError::Io(format!("{error}; cache read failed: {e}")))?,
            Err(error) => return Err(VersionError::Network(error.to_string())),
        }
    };
    let mut manifest: VersionManifest = serde_json::from_slice(&raw).map_err(|e| VersionError::InvalidManifest(e.to_string()))?;
    if !include_snapshots { manifest.versions.retain(|version| version.kind == "release"); }
    Ok(manifest)
}
