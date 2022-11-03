use chrono::{DateTime, Utc};

/// Represents a user of Mastodon and their associated profile
pub struct Account {
    /// The account id header
    pub id: String,
    /// The username of the account, not including domain
    pub username: String,
    /// The Webfinger account URI. Equal to username for local users,
    /// or username@domain for remote users
    pub acct: String,
    /// The location of the user's profile page
    pub url: String,
    /// The profile's display name
    pub display_name: String,
    /// The profile's bio / description
    pub note: String,
    /// An image icon that is shown next to statuses and in the
    /// profile
    pub avatar: String,
    /// A static version of the avatar. Equal to avatar if its value
    /// is a static image; different if avatar is an animated GIF
    pub avatar_static: String,
    /// An image banner that is shown above the profile and in profile cards
    pub header: String,
    /// A static version of the header. Equal to header if its value is a
    /// static image; different if header is an animated GIF
    pub header_static: String,
    /// Whether the account manually approves follow requests
    pub locked: bool,
    /// Custom emoji entities to be used when rendering the profile. If none,
    /// an empty array will be returned
    // TODO: Replace String with Emoji type
    pub emojis: Vec<String>,
    /// Whether the account has opted into discovery features such as the
    /// profile directory
    pub discoverable: bool,
    /// When the account was created
    pub created_at: DateTime<Utc>,
    /// When the most recent status was posted
    pub last_status_at: DateTime<Utc>,
    /// How many statuses are attached to this account
    pub statuses_count: i64,
    /// The reported followers of this profile
    pub followers_count: i64,
    /// The reported follows of this profile
    pub following_count: i64,
    /// Indicates that the profile is currently inactive and that its user
    /// has moved to a new account
    pub moved: Box<Account>,
    /// Additional metadata attached to a profile as name-value pairs
    // TODO: Replace String with Field type
    pub fields: Vec<String>,
    /// A presentational flag. Indicates that the account may perform automated
    /// actions, may not be monitored, or identifies as a robot.
    pub bot: bool,
    /// An extra entity to be used with API methods to verify_credentials and
    /// update_credentials
    // TODO: Replace String with Source type
    pub source: String,
    /// An extra entity returned when an account is suspended
    pub suspended: bool,
    /// When a timed mute will expire, if applicable
    pub mute_expires_at: DateTime<Utc>,
}
