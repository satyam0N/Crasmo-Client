//! Mojang version manifest integration boundary.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum VersionError { Network(String), InvalidManifest(String) }
