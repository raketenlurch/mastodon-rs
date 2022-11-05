pub struct Error {
    /// The error message
    pub error: Reason,
    /// A longer description of the error, mainly provided
    /// with the OAuth API
    pub error_description: Option<String>,
}

pub enum Reason {
    /// This API requires an authenticated user. Appears when
    /// the instance is in secure mode, which disables all
    /// public use of API methods
    RequireAuthenticatedUser,
    /// Your login is currently disabled. Appears when the OAuth
    /// token's authorized user has had their account disabled
    /// by a moderator
    CurrentUserDisabled,
    /// Your login is missing a confirmed e-mail address. Appears
    /// when the email address associated with the OAuth token's
    /// authorized user's account has not yet been confirmed
    CurrentUserConfirmed,
    /// Your login is currently pending approval. Appears when the
    /// OAuth token's authorized user has signed up on an instance
    /// with approval-required registrations, and the user has not
    /// yet had their account approved by a moderator
    CurrentUserApproved,
    /// Record not found. Appears when an enity record does not exist,
    /// or the authorized user is not within the audience of a private
    /// entity. Operates on a user
    RecordNotFound,
    /// {string}. May appear when entity creation failed
    RecordInvalid,
    /// Duplicate record. Appears when you are trying to pin an account
    /// or status that is already pinned
    RecordNotUnique,
    /// This method requires an authenticated user. Appears when using an
    /// OAuth token without an authorized user (or no token at all), while
    /// trying to call an API method that requires a user to be processed
    CurrentUser,
}
