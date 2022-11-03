/// Represents an emoji reaction to an Announcement
pub struct AnnouncementReaction {
    /// The emoji used for the reaction. Either a unicode emoji, or a
    /// custom emoji's shortcode
    pub name: String,
    /// The total number of users who have added this reaction
    pub count: i64,
    /// Whether the authorized user has added this reaction to the announcement
    pub me: bool,
    /// A link to the custom emoji
    pub url: String,
    /// A link to a non-animated version of the custom emoji
    pub static_url: String,
}
