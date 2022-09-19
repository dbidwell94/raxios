use crate::RaxiosHeaders;

#[derive(Default, Debug, Clone)]
pub struct RaxiosConfig {
    pub timeout_ms: Option<usize>,
    pub headers: Option<RaxiosHeaders>,
}
