/// Represents a weekly bucket of instance activity
pub struct Activity {
    /// Midnight at the first day of the week
    // TODO: Replace String with a more useful representation of the
    // Unix timestamp
    pub week: String,
    /// Statuses created since the week began
    pub statuses: String,
    /// User logins since the week began
    pub logins: String,
    /// User registrations since the week began
    pub registrations: String,
}
