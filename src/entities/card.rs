/// Represents a rich preview card that is generated using
/// OpenGraph tags from a URL
pub struct Card {
    /// Location of linked resource
    pub url: String,
    /// Title of linked resource
    pub title: String,
    /// Description of preview
    pub description: String,
    /// The type of the preview card
    pub card_type: Type,
    /// The author of the original resource
    pub author_name: Option<String>,
    /// A link to the author of the original resource
    pub author_url: Option<String>,
    /// The provider of the original resource
    pub provider_name: Option<String>,
    /// A link to the provider of the original resource
    pub provider_url: Option<String>,
    /// HTML to be used for generating the preview card
    pub html: Option<String>,
    /// Width of preview, in pixels
    pub width: Option<u64>,
    /// Height of preview, in pixels
    pub height: Option<u64>,
    /// Preview thumbnail
    pub image: Option<String>,
    /// Used for photo embeds, instead of custom html
    pub embed_url: Option<String>,
    /// A hash computed by the BlurHash algorithm, for
    /// generating colorful preview thumbnails when
    /// media has not been downloaded yet
    pub blurhash: Option<String>,
}

/// The type of the preview card
pub enum Type {
    /// Link OEmbed
    Link,
    /// Photo OEmbed
    Photo,
    /// Video OEmbed
    Video,
    /// iframe OEmbed. Not currently accepted,
    /// so won't show up in pratice
    Rich,
}
