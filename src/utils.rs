use std::{collections::HashMap, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName};

use crate::{
    error::{RaxiosError, RaxiosResult},
    RaxiosHeaders,
};

/// A quick macro to generate a HashMap<String, String>
/// 
/// # Example 1
/// ```rust
///     // The type declaration is for the example only and is not needed.
///     let style1: std::collections::HashMap<String, String> = raxios::map_string!{item1 : "item", item2 : "item"};
/// ```
/// 
/// # Example 2
/// ```rust
///     // The type declaration is for the example only and is not needed.
///     let style2: std::collections::HashMap<String, String> = raxios::map_string!{"item1" => "item", "item2" => "item"};
/// ```
/// 
#[macro_export]
macro_rules! map_string {
    ($($key:ident : $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut h_map: ::std::collections::HashMap<String, String> = ::std::collections::HashMap::new();
        $(
            h_map.insert(stringify!($key).to_string(), $value.to_string());
        )*
        h_map
    }};

    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut h_map: ::std::collections::HashMap<String, String> = ::std::collections::HashMap::new();
        $(
            h_map.insert($key.to_string(), $value.to_string());
        )*
        h_map
    }};
}

pub fn reqwest_headers_to_map(header: &HeaderMap) -> RaxiosResult<RaxiosHeaders> {
    let mut to_return: RaxiosHeaders = HashMap::new();
    for (key, value) in header {
        to_return.insert(
            key.to_string(),
            value
                .to_str()
                .map_err(|e| RaxiosError::Unknown(anyhow::anyhow!(e)))?
                .to_owned(),
        );
    }

    return Ok(to_return);
}

pub fn map_to_reqwest_headers(map: &RaxiosHeaders) -> RaxiosResult<HeaderMap> {
    let mut headers = reqwest::header::HeaderMap::new();
    for (key, value) in map {
        headers.insert(
            HeaderName::from_str(key)
                .map_err(|_| RaxiosError::HeaderParseError(key.to_owned(), value.to_owned()))?,
            value
                .parse()
                .map_err(|_| RaxiosError::HeaderParseError(key.to_owned(), value.to_owned()))?,
        );
    }
    Ok(headers)
}

#[cfg(test)]
mod utils_tests {
    use super::map_to_reqwest_headers;

    #[test]
    fn test_map_macro_ident() {
        let map = map_string! {testas : "test", test2 : "test2"};

        assert_eq!(2, map.len());
    }

    #[test]
    fn test_map_macro_expr() {
        let map = map_string! {"test" => "testing123", "test2" => "testing123"};

        assert_eq!(2, map.len());
    }

    #[test]
    fn test_no_args_macro() {
        let map = map_string! {};
        assert_eq!(0, map.len());
    }

    #[test]
    fn test_map_to_reqwest_headers() {
        let header_map = map_string! {
            value1 : "value",
            value2 : "value",
            value3 : "value"
        };
        let headers = map_to_reqwest_headers(&header_map);
        assert_ne!(true, headers.is_err());
    }
}
