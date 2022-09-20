use crate::RaxiosHeaders;

#[derive(Default, Debug, Clone)]
pub struct RaxiosConfig {
    pub timeout_ms: Option<u64>,
    pub headers: Option<RaxiosHeaders>,
}
