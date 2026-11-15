//! Modrinth integration boundary.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum ModError { Network(String), InvalidProject(String), Install(String) }
