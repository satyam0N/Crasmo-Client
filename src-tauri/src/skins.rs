//! Minecraft skin service integration boundary; never stores account secrets here.
#[derive(Debug, serde::Serialize)]
#[serde(tag="code", content="message")]
pub enum SkinError { InvalidPng(String), AuthenticationRequired, Provider(String) }
