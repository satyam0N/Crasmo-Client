//! Instance persistence is planned under the app data directory.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum InstanceError { Io(String), InvalidInput(String), NotFound(String) }
