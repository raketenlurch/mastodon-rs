use chrono::{DateTime, Utc};

/// Represents a proof from an external identity provider
pub struct IdentityProof {
    /// The name of the identity provider
    pub provider: String,
    /// The account owner's username on the identity provider's service
    pub provider_username: String,
    /// The account owner's profile URL on the identity provider.
    pub profile_url: String,
    /// A link to a statement of identity proof, hosted by the identity provider.
    pub proof_url: String,
    /// When the identity proof was last updated
    pub updated_at: DateTime<Utc>,
}
