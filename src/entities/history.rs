/// Represents daily usage history of a hashtag
pub struct History {
    /// UNIX timestamp on midnight of the given day
    pub day: String,
    /// The counted usage of the tag within that day
    pub uses: String,
    /// The total of accounts using the tag within that day
    pub accounts: String,
}
