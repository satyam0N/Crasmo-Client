//! Authentication integration boundary. OAuth and credential persistence are intentionally not stubbed as successful.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum AuthError { NotConfigured(String), Provider(String), Keychain(String) }
