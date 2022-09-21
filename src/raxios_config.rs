use crate::{RaxiosHeaders, raxios_options::ContentType};

#[derive(Default, Debug, Clone)]
pub struct RaxiosConfig {
    pub timeout_ms: Option<u64>,
    pub headers: Option<RaxiosHeaders>,
    /// What content-type should these requests accept (overrideable via RaxiosOptions)
    pub accept: ContentType,
    /// What content-type does do these requests send (overrideable via RaxiosOptions)
    pub content_type: ContentType
}
