
# Raxios

Async-centered Rust library similar to the JS library "axios"


## Features

- JSON, XML, and URL-Encoded Serialization
- JSON Deserialization (XML and others to come)
- An "axios"-like api


## Usage/Examples

```rust
use raxios::Raxios;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ToRecieve {
    field1: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ToSend {
    field1: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // An error _might_ occur here if you try to set non-valid headers in the Options
    let client = Raxios::new("", None)?;

    let data_to_send = ToSend { field1 : String::from("Hello World") };

    let result = client
        .post::<ToRecieve, ToSend>("/endpoint", Some(data_to_send), None)
        .await?;

    println!("{0}", result.body.unwrap());
}
```


## Documentation

[docs.rs](https://docs.rs/raxios)


## License

[MIT](https://choosealicense.com/licenses/mit/)


|                      docs.rs                   |                     downloads                      |                         Version                      |
| ---------------------------------------------- | -------------------------------------------------- | ---------------------------------------------------- |
|![docs.rs](https://img.shields.io/docsrs/raxios)|![Crates.io](https://img.shields.io/crates/d/raxios)| ![Crates.io](https://img.shields.io/crates/v/raxios) |
