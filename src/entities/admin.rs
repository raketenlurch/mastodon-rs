use super::account;
use chrono::{DateTime, Utc};

/// Admin-level information about a given account
pub struct Account {
    /// The ID of the account in the database
    pub id: String,
    /// The username of the account
    pub username: String,
    /// The domain of the account
    pub domain: String,
    /// When the account was first discovered
    pub created_at: DateTime<Utc>,
    /// The email address associated with the account
    pub email: String,
    /// The IP address last used to login to this account
    pub ip: String,
    /// The locale of the account
    pub locale: String,
    /// Invite request text
    pub invite_request: String,
    /// The current role of the account
    pub role: String,
    /// Whether the account has confirmed their email address
    pub confirmed: bool,
    /// Whether the account is currently approved
    pub approved: bool,
    /// Whether the account is currently disabled
    pub disabled: bool,
    /// Whether the account is sileneced
    pub silenced: bool,
    /// Whether the account is currently suspended
    pub suspended: bool,
    /// User-level information about the account
    pub account: account::Account,
    /// The ID of the application that created this account
    pub created_by_application_id: String,
    /// The ID of the account that invited this user
    pub invited_by_account_id: String,
}

/// Admin-level information about a filed report
pub struct Report {
    /// The ID of the report in the database
    pub id: String,
    /// The action taken to resolve this report
    pub action_taken: String,
    /// An optional reason for reporting
    pub comment: String,
    /// The time the report was filed
    pub created_at: DateTime<Utc>,
    /// The time of last action on this report
    pub updated_at: DateTime<Utc>,
    /// The account which filed the report
    pub account: account::Account,
    /// The account being reported
    pub target_account: account::Account,
    /// The account of the moderator assigned to this report
    pub assigned_account: account::Account,
    /// The action taken by the moderator who handled the report
    pub action_taken_by_account: String,
    /// Statuses attached to the report, for context
    // TODO: Replace String with Status type
    pub statuses: Vec<String>,
}
