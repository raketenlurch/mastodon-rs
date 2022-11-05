/// Represents a custom emoji
pub struct Emoji {
    /// The name of the custom emoji
    pub shortcode: String,
    /// A link to the custom emoji
    pub url: String,
    /// A link to a static copy of the custom emoji
    pub static_url: String,
    /// Whether the emoji should be visible in the
    /// picker or unlisted
    pub visible_in_picker: bool,
    /// Used for sorting custom emoji in the picker
    pub category: Option<String>,
}
