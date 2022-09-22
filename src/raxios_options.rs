use crate::RaxiosHeaders;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct RaxiosOptions {
    pub headers: Option<RaxiosHeaders>,
    pub accept: Option<ContentType>,
    pub content_type: Option<ContentType>,
    pub params: Option<HashMap<String, String>>,
    pub deserialize_body: bool,
}

impl Default for RaxiosOptions {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            params: Default::default(),
            accept: Default::default(),
            content_type: Default::default(),
            deserialize_body: true,
        }
    }
}

#[derive(Debug, Clone)]
/// Suppling Raxios with a ContentType will set the `content-type` header as well as change how the data is serialized to the server
pub enum ContentType {
    /// Serialize as `application/json`
    Json,
    /// Serialize as `text/xml`
    TextXml,
    /// Serialize as `application/xml`
    ApplicationXml,
    /// Serialize as `application/x-www-form-urlencoded`
    UrlEncoded,
}

impl FromStr for ContentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "application/json" => Ok(Self::Json),
            "text/xml" => Ok(Self::TextXml),
            "application/xml" => Ok(Self::ApplicationXml),
            "application/x-www-form-urlencoded" => Ok(Self::UrlEncoded),
            _ => Err(()),
        };
    }
}

impl From<ContentType> for String {
    fn from(content_type: ContentType) -> Self {
        format!("{content_type}")
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Json => write!(f, "application/json"),
            ContentType::TextXml => write!(f, "text/xml"),
            ContentType::ApplicationXml => write!(f, "application/xml"),
            ContentType::UrlEncoded => write!(f, "application/x-www-form-urlencoded"),
        }
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Json
    }
}

#[cfg(test)]
mod tests {
    use super::ContentType;

    #[test]
    fn test_content_type_json_to_string() {
        let c_type = ContentType::Json;
        assert_eq!(String::from("application/json"), String::from(c_type));
    }

    #[test]
    fn test_content_type_application_xml_to_string() {
        let c_type = ContentType::ApplicationXml;
        assert_eq!(String::from("application/xml"), String::from(c_type));
    }

    #[test]
    fn test_content_type_text_xml_to_string() {
        let c_type = ContentType::TextXml;
        assert_eq!(String::from("text/xml"), String::from(c_type));
    }

    #[test]
    fn test_content_type_url_encoded_to_string() {
        let c_type = ContentType::UrlEncoded;
        assert_eq!(
            String::from("application/x-www-form-urlencoded"),
            String::from(c_type)
        );
    }
}
