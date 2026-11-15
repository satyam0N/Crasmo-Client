//! Typed persistent configuration boundary.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings { pub memory_min_gb: u8, pub memory_max_gb: u8, pub memory_recommended_gb: u8, pub close_on_launch: bool, pub show_snapshots: bool }
impl Default for Settings { fn default() -> Self { Self { memory_min_gb: 2, memory_max_gb: 16, memory_recommended_gb: 6, close_on_launch: false, show_snapshots: false } } }
impl Settings { pub fn validate(&self) -> Result<(), &'static str> { if self.memory_min_gb < 1 || self.memory_max_gb > 128 || self.memory_min_gb > self.memory_max_gb || !(self.memory_min_gb..=self.memory_max_gb).contains(&self.memory_recommended_gb) { return Err("Invalid memory configuration"); } Ok(()) } }
