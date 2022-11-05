/// Represents the tree around a given status.
/// Used for reconstructing threads of statuses
pub struct Context {
    /// Parents in the thread
    // FIXME: Replace String with Status type
    pub ancestors: Vec<String>,
    /// Children in the thread
    // FIXME: Replace String with Status type
    pub descendants: Vec<String>,
}
