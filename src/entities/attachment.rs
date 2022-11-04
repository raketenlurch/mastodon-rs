/// Represents a file or media attachment that can be
/// added to a status
pub struct Attachment {
    /// The ID of the attachment in the database
    pub id: String,
    /// The type of the attachment
    pub attachment_type: Type,
    /// The location of the original full-size attachment
    pub url: String,
    /// The location of a scaled-down preview of the attachment
    pub preview_url: String,
    /// The location of the full-sized original attachment on
    /// the remote website
    pub remote_url: Option<String>,
    /// Metadata returned by Paperclip
    // FIXME: Change Entity type to Hash
    pub meta: Option<String>,
    /// Alternate text that describes what is in the media attachment,
    /// to be used for the visually impaired or when media attachments
    /// do not load
    pub description: Option<String>,
    /// A hash computed by the BlurHash algorithm, for generating
    /// colorful preview thumpnails when media has not been
    /// downloaded yet
    pub blurhash: Option<String>,
    /// A shorter URL for the attachment
    // FIXME: deprecated
    pub text_url: String,
}

pub enum Type {
    /// Unsupported or unrecognized file type
    Uknown,
    /// Static image
    Image,
    /// Looping, soundless animation
    Gifv,
    /// Video clip
    Video,
    /// Audio track
    Audio,
}
