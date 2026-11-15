//! JVM argument construction and child-process supervision boundary.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum LaunchError { JavaNotFound, InvalidVersion(String), Spawn(String) }
