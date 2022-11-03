/// Represents an application that interfaces with the REST API
/// to access accounts or post statuses
pub struct Application {
    /// The name of your application
    pub name: String,
    /// The website assiciated with your application
    pub website: Option<String>,
    /// Used for Push Streaming API. Returned with
    /// POST /api/v1/apps. Equivalent to PushSubscription#server_key.
    pub vapid_key: Option<String>,
    /// Client ID key, to be used for obtaining OAuth tokens
    pub client_id: String,
    /// Client secret key, to be used for obtaining OAuth tokens
    pub client_secret: String,
}
