/// Represents a rich preview card that is generated using 
/// OpenGraph tags from a URL
pub struct Card {
    /// Location of linked resource
    pub url: String,
    /// Title of linked resource
    pub title: String,
    /// Description of preview
    pub description: String,
    /// 
    pub card_type: Type,
    pub
    pub
    pub
    pub
    pub
    pub
    pub
    pub
    pub
    pub
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