use chrono::{DateTime, Utc};

/// Represents a user-defined filter for determining which statuses
/// should not be shown to the user
///
/// # Implementation notes
/// If *whole_word* is true, client app should do:
///
/// - Define *'word constituent character'* for your app. In the official
///   implementation, it's *\[A-Za-z0-9\_\]* in JavaScript, and *\[\[:word:\]\]*
///   in Ruby. Ruby uses the POSIX character class
///   (Letter|Mark|Decimal_Number|Connector_Punctuation)
///
/// - If the phrase starts with a word character, and if the previous character
///   before matched range is a word character, its matched range should be
///   treated to not match
///
/// - If the phrase ends with a word character, and if the next character after
///   matched range is a word character, its matched range should be treated to
///   not match
///
/// Please check *app/javascript/mastodon/selectors/index.js* and *app/lib/feed_manager.rb*
/// in the Mastodon source code for more details.
pub struct Filter {
    /// The ID of the filter in the database
    pub id: String,
    /// The text to be filtered
    pub phrase: String,
    /// The contexts in which the filter should be applied
    pub context: Vec<ContextType>,
    /// When the filter should no longer be applied
    pub expires_at: Option<DateTime<Utc>>,
    /// Should matching entities in home and notifications
    /// be dropped by the server?
    pub irreversible: bool,
    /// Should the filter consider word boundaries?
    pub whole_word: bool,
}

pub enum ContextType {
    /// Home timeline and lists
    Home,
    /// Notification timeline
    Notifications,
    /// Public timelines
    Public,
    /// Expanded thread of a detailed status
    Thread,
}
