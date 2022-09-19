use std::collections::HashMap;

use crate::RaxiosHeaders;

#[derive(Default, Debug, Clone)]
pub struct RaxiosOptions {
    pub headers: Option<RaxiosHeaders>,
    pub params: Option<HashMap<String, String>>,
}