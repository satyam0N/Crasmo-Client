//! Download orchestration boundary. Hash verification should be required before install.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum DownloadError { Network(String), HashMismatch(String), Cancelled, Io(String) }
