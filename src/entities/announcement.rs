use super::announcement_reaction::AnnouncementReaction;
use chrono::{DateTime, Utc};

/// Represents an announcement set by an administrator
pub struct Announcement {
    /// The announcement id
    pub id: String,
    /// The content of the announcement
    pub text: String,
    /// Whether the announcement is currently active
    pub published: bool,
    /// Whether the announcement has a start/end time
    pub all_day: bool,
    /// When the announcement was created
    pub created_at: DateTime<Utc>,
    /// When the announcement was last updated
    pub updated_at: DateTime<Utc>,
    /// Whether the announcement has been read by the user
    pub read: bool,
    /// Emoji reactions attached to the announcement
    pub reactions: Vec<AnnouncementReaction>,
    /// When the future announcement was scheduled
    pub scheduled_at: DateTime<Utc>,
    /// When the future announcement will start
    pub starts_at: DateTime<Utc>,
    /// When the future announcement will end
    pub ends_at: DateTime<Utc>,
}
