use super::account::Account;

/// Represents the software instance of Mastodon running
/// on this domain
pub struct Instance {
    /// The domain name of the instance
    pub uri: String,
    /// The title of the website
    pub title: String,
    /// Admin-defined description of the Mastodon site
    pub description: String,
    /// A shorter description defined by the admin
    pub short_description: String,
    /// An email that may be contacted for any inquiries
    pub email: String,
    /// The version of Mastodon installed on the instance
    pub version: String,
    /// Primary languages of the website and its staff
    pub languages: Vec<String>,
    /// Whether registrations are enabled
    pub registrations: bool,
    /// Whether registrations require moderator approval
    pub approval_required: bool,
    /// Whether invites are enabled
    pub invites_enabled: bool,
    /// URLs of interest for clients apps
    // FIXME: Replace String with Hash type
    pub urls: String,
    /// Statistics about how much information the instance contains
    // FIXME: Replace String with Hash type
    pub stats: String,
    /// Banner image for the website
    pub thumbnail: Option<String>,
    /// A user that can be contacted, as an alternative to *email*
    pub contact_account: Option<Account>,
}
