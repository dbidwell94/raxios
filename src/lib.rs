mod error;
mod network_error;
mod raxios_config;
mod raxios_options;
mod raxios_response;
mod utils;
use anyhow::anyhow;
pub use error::{RaxiosError, RaxiosResult};
use network_error::NetworkError;
pub use raxios_config::RaxiosConfig;
pub use raxios_options::RaxiosOptions;
pub use raxios_response::RaxiosResponse;
pub use reqwest;
use reqwest::{header::HeaderMap, Client, ClientBuilder, RequestBuilder, Response, Url};
use serde::{Deserialize, Serialize};
use utils::{map_to_reqwest_headers, reqwest_headers_to_map};

pub type RaxiosHeaders = ::std::collections::HashMap<String, String>;

#[derive(Default, Debug)]
pub struct Raxios {
    client: Client,
    config: Option<RaxiosConfig>,
    base_url: String,
}

impl Raxios {
    /// Creates a new instance of Raxios with a set base url and optional Options
    ///
    /// # Example
    /// ```rust
    /// use raxios::Raxios;
    /// let client = Raxios::new("http://localhost", None);
    /// assert_ne!(true, client.is_err());
    ///
    /// ```
    pub fn new(base_url: &str, options: Option<RaxiosConfig>) -> RaxiosResult<Self> {
        let default_headers: HeaderMap;
        let mut client = ClientBuilder::default();
        if let Some(options) = &options {
            if let Some(headers) = &options.headers {
                default_headers = map_to_reqwest_headers(headers)?;
                client = client.default_headers(default_headers);
            }
        }

        Ok(Self {
            base_url: base_url.to_string(),
            config: options,
            client: client
                .build()
                .map_err(|e| RaxiosError::Unknown(anyhow!(e)))?,
            ..Default::default()
        })
    }

    /// Sets the default headers for this instance of Raxios.
    ///
    /// # Example
    /// ```rust
    /// use raxios::{Raxios, map_string};
    ///
    /// let mut client = Raxios::new("http://localhost", None).unwrap();
    /// let set_header_result = client.set_default_headers(Some(map_string!{ header1 : "header 1 value" }));
    /// assert_ne!(true, set_header_result.is_err());
    ///
    /// ```
    pub fn set_default_headers(&mut self, headers: Option<RaxiosHeaders>) -> RaxiosResult<()> {
        let opts: RaxiosConfig = RaxiosConfig {
            headers,
            ..self.config.clone().unwrap_or(Default::default())
        };

        let new_raxios = Self::new(&self.base_url, Some(opts))?;
        self.client = new_raxios.client;
        self.config = new_raxios.config;

        Ok(())
    }

    fn build_url(&self, endpoint: &str, options: Option<&RaxiosOptions>) -> RaxiosResult<Url> {
        let mut built_string = String::new();
        built_string += &self.base_url;

        if built_string.chars().nth(built_string.len() - 1) != Some('/')
            && endpoint.chars().nth(0) != Some('/')
        {
            built_string += "/";
        }

        built_string += endpoint;
        if let Some(options) = options {
            if let Some(params) = &options.params {
                let mut added_param = false;
                for (index, (key, value)) in params.iter().enumerate() {
                    if !added_param {
                        built_string += "?";
                        added_param = true;
                    }
                    built_string += &format!("{key}={value}");
                    if index < params.len() - 1 {
                        built_string += "&";
                    }
                }
            }
        }

        let url: Url = built_string
            .parse()
            .map_err(|_| RaxiosError::InvalidUrl(built_string))?;

        Ok(url)
    }

    fn build_request<U>(
        &self,
        data: Option<U>,
        options: Option<&RaxiosOptions>,
        original_builder: RequestBuilder,
    ) -> RaxiosResult<RequestBuilder>
    where
        U: Serialize,
    {
        let mut builder = original_builder;
        if let Some(body) = data {
            builder = builder.json(&body);
        }
        if let Some(options) = options {
            if let Some(headers) = &options.headers {
                builder = builder.headers(map_to_reqwest_headers(headers)?);
            }
        };
        return Ok(builder);
    }

    fn check_response_and_return_err(&self, response: Response) -> RaxiosResult<Response> {
        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(RaxiosError::NetworkError(NetworkError::from(response)));
        }
        Ok(response)
    }

    async fn response_to_raxios_response<T>(
        &self,
        response: Response,
        deserialize_body: bool,
    ) -> RaxiosResult<RaxiosResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.check_response_and_return_err(response)?;

        let headers = response.headers().clone();
        let remote_address = response.remote_addr();
        let status = response.status();

        let raw_body = response.bytes().await.ok();
        let mut body: Option<T> = None;

        if let Some(raw_body) = &raw_body {
            body = serde_json::from_slice::<T>(raw_body).ok();
        }

        return Ok(RaxiosResponse {
            body,
            raw_body,
            status,
            headers: reqwest_headers_to_map(&headers)?,
            remote_address,
        });
    }

    /// Sends an HTTP Post request to the configured remote server
    ///
    /// * `endpoint` - The remote endpoint. This gets joined with the base_url configured in the ::new() method
    /// * `data` - Optional data to send to the remote endpoint (to be serialized as JSON). If `None`, then no data is sent instead of `null`
    /// * `options` - The `RaxiosOptions` for this call. Allows setting of headers and/or query params
    ///
    /// # Example
    /// ```rust
    /// use httpmock::prelude::*;
    /// use raxios::Raxios;
    ///
    /// #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    /// struct ToReturn {}
    ///
    /// #[derive(serde::Serialize, serde::Deserialize, Debug)]
    /// struct ToSend {
    ///     testKey: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let server = MockServer::start();
    ///
    ///     server.mock(|when, then| {
    ///         when.path("/test").method(POST);
    ///         then.status(200).json_body(serde_json::json!({}));
    ///     });
    ///
    ///     let raxios = Raxios::new(&server.base_url(), None).unwrap();
    ///
    ///      let response = raxios
    ///         .post::<ToReturn, ToSend>(
    ///             "/test",
    ///             Some(ToSend {
    ///                 testKey: "Testing".to_string(),
    ///             }),
    ///             Some(raxios::RaxiosOptions {
    ///                 params: Some(raxios::map_string! {param1 : "value1"}),
    ///                 ..Default::default()
    ///             }),
    ///         )
    ///         .await?;
    ///     assert_eq!(&200, &response.status);
    ///     assert_eq!(ToReturn {}, response.body.unwrap());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn post<T, U>(
        &self,
        endpoint: &str,
        data: Option<U>,
        options: Option<RaxiosOptions>,
    ) -> RaxiosResult<RaxiosResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
        U: Serialize,
    {
        let options = options.unwrap_or_default();
        let response = self
            .build_request(
                data,
                Some(&options),
                self.client.post(self.build_url(endpoint, Some(&options))?),
            )?
            .send()
            .await
            .map_err(|e| RaxiosError::UnableToSendRequest { err: e })?;

        return Ok(self
            .response_to_raxios_response(response, options.deserialize_body)
            .await?);
    }

    /// Sends an HTTP GET request to the configured remote server
    ///
    /// * `endpoint` - The remote endpoint. This gets joined with the base_url configured in the ::new() method
    /// * `options` - The `RaxiosOptions` for this call. Allows setting of headers and/or query params
    ///
    /// # Example
    ///
    /// ```rust
    ///     use raxios::Raxios;
    ///     use httpmock::prelude::*;
    ///
    ///     #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    ///     struct ToReturn {
    ///         
    ///     }
    ///
    ///     #[tokio::main]
    ///     async fn main() -> anyhow::Result<()> {
    ///         let server = MockServer::start();
    ///         
    ///         server.mock(|when, then|{
    ///             when.path("/test");
    ///             then.status(200).json_body(serde_json::json!({}));
    ///         });
    ///
    ///         let raxios = Raxios::new(&server.base_url(), None).unwrap();
    ///
    ///         let response = raxios.get::<ToReturn>("/test", Some(raxios::RaxiosOptions
    ///         {
    ///             params: Some(raxios::map_string!{param1 : "value1"}),
    ///             ..Default::default()
    ///         })).await?;
    ///         assert_eq!(&200, &response.status);
    ///         assert_eq!(ToReturn{}, response.body.unwrap());
    ///
    ///         Ok(())
    ///     }
    ///     
    /// ```
    pub async fn get<T>(
        &self,
        endpoint: &str,
        options: Option<RaxiosOptions>,
    ) -> RaxiosResult<RaxiosResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let options = options.unwrap_or_default();
        let response = self
            .build_request::<()>(
                None,
                Some(&options),
                self.client.get(self.build_url(endpoint, Some(&options))?),
            )?
            .send()
            .await
            .map_err(|e| RaxiosError::UnableToSendRequest { err: e })?;

        return Ok(self
            .response_to_raxios_response(response, options.deserialize_body)
            .await?);
    }

    /// Sends an HTTP DELETE request to the configured remote server
    pub async fn delete<U>(
        &self,
        endpoint: &str,
        options: Option<RaxiosOptions>,
    ) -> RaxiosResult<RaxiosResponse<U>>
    where
        U: for<'de> Deserialize<'de>,
    {
        let options = options.unwrap_or_default();
        let response = self
            .build_request::<()>(
                None,
                Some(&options),
                self.client
                    .delete(self.build_url(endpoint, Some(&options))?),
            )?
            .send()
            .await
            .map_err(|e| RaxiosError::UnableToSendRequest { err: e })?;

        return Ok(self
            .response_to_raxios_response(response, options.deserialize_body)
            .await?);
    }
}

#[cfg(test)]
mod raxios_tests {
    use crate::{map_string, raxios_options::RaxiosOptions, Raxios};
    use httpmock::prelude::*;

    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct NetworkTestResponse {
        item1: String,
        item2: String,
    }

    #[test]
    fn test_set_default_headers() {}

    #[test]
    fn test_build_url_leading_slash() {
        let raxios = Raxios::new("http://localhost", None).unwrap();

        let built_url = raxios.build_url("/v1/signup", None).unwrap();
        assert_eq!("http://localhost/v1/signup", built_url.as_str());
    }

    #[test]
    fn test_build_url_no_leading_slash() {
        let raxios = Raxios::new("http://localhost", None).unwrap();
        let built_url = raxios.build_url("v1/signup", None).unwrap();

        assert_eq!("http://localhost/v1/signup", built_url.as_str());
    }

    #[test]
    fn test_build_url_with_params() {
        let raxios = Raxios::new("http://localhost", None).unwrap();
        let built_url = raxios
            .build_url(
                "/v1/signup",
                Some(&RaxiosOptions {
                    params: Some(map_string! {param1 : "testParam1"}),
                    ..Default::default()
                }),
            )
            .unwrap();

        assert_eq!(
            "http://localhost/v1/signup?param1=testParam1",
            built_url.as_str()
        );
    }

    #[tokio::test]
    async fn test_raxios_post() {
        let server = MockServer::start();
        let raxios = Raxios::new(&server.base_url(), None).unwrap();

        server.mock(|when, then| {
            when.method(POST).path("/test");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&NetworkTestResponse {
                        item1: "Test".to_owned(),
                        item2: "Test2".to_owned(),
                    })
                    .unwrap(),
                );
        });

        let response = raxios
            .post::<NetworkTestResponse, ()>("/test", None, None)
            .await;

        assert_eq!(false, response.is_err());

        let response = response.unwrap();
        assert_eq!(&200, &response.status);
        assert_eq!(
            &NetworkTestResponse {
                item1: "Test".to_owned(),
                item2: "Test2".to_owned()
            },
            &response.body.unwrap()
        );
        assert_eq!(
            "application/json",
            response.headers.get("content-type").unwrap()
        );
    }

    #[tokio::test]
    async fn test_raxios_get() {
        let server = MockServer::start();
        let raxios = Raxios::new(&server.base_url(), None).unwrap();

        let test_response = NetworkTestResponse {
            item1: "test1".to_string(),
            item2: "test2".to_string(),
        };

        server.mock(|when, then| {
            when.path("/test").method(GET).query_param("key", "value");
            then.body(serde_json::to_string(&test_response).unwrap())
                .status(200);
        });

        let response = raxios
            .get::<NetworkTestResponse>(
                "/test",
                Some(RaxiosOptions {
                    headers: None,
                    params: Some(map_string! {key : "value"}),
                    ..Default::default()
                }),
            )
            .await;

        assert_ne!(true, response.is_err());
        let response = response.unwrap();

        assert_eq!(&200, &response.status);
        assert_eq!(&test_response, &response.body.unwrap());
    }
}
