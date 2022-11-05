use chrono::{DateTime, Utc};

/// Represents a profile field as a name-value
/// pair with optional verification
pub struct Field {
    /// The key of a given field's key-value pair
    pub name: String,
    /// The value associated with the name key
    pub value: String,
    /// Timestamp of when the server verified a URL
    /// value for a rel="me" link
    pub verified_at: Option<DateTime<Utc>>,
}
