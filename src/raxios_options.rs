use std::collections::HashMap;

use crate::RaxiosHeaders;

#[derive(Debug, Clone)]
pub struct RaxiosOptions {
    pub headers: Option<RaxiosHeaders>,
    pub params: Option<HashMap<String, String>>,
    pub deserialize_body: bool,
}

impl Default for RaxiosOptions {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            params: Default::default(),
            deserialize_body: true,
        }
    }
}
