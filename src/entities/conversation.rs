/// Represents a conversation with "direct message" visibility
pub struct Conversation {
    /// Local database ID of the conversation
    pub id: String,
    /// Participants in the conversation
    // FIXME: Replace String with Account type
    pub accounts: Vec<String>,
    /// Is the conversation currently marked as unread?
    pub unread: bool,
    /// The last status in the conversation, to be
    /// used for optional display
    // FIXME: Replace String with Status type
    pub last_status: Option<String>,
}
