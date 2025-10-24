//! Shared types and utilities for Pluck

/// A clipboard entry
pub struct Clip {
    pub content: String,
    pub timestamp: String, // Could later be a chrono type
}

impl Clip {
    pub fn new(content: &str) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            content: content.to_string(),
            timestamp: now,
        }
    }
}
