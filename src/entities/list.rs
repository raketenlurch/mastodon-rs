/// Represents a list of some users that the authenticated
/// user follows
pub struct List {
    /// The internal database ID of the list
    pub id: String,
    /// The user-defined title of the list
    pub title: String,
    /// The user-defined title of the list
    pub replies_policy: ReplyType,
}

pub enum ReplyType {
    /// Show replies to any followed user
    Followed,
    /// Show replies to members of the list
    List,
    /// Show replies to no one
    NoOne,
}
