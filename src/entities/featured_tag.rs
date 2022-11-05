use chrono::{DateTime, Utc};

/// Represents a hashtag that is featured on a profile
pub struct FeaturedTag {
    /// The internal ID of the featured tag in the database
    pub id: String,
    /// The name of the hashtag being featured
    pub name: String,
    /// A link to all statuses by a user that contain this hashtag
    pub url: String,
    /// The number of authored statuses containing this hashtag
    pub statuses_count: u64,
    /// The timestamp of the last authored status containing this
    /// hashtag
    pub last_status_at: DateTime<Utc>,
}
