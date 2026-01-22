# ctes_models for Rust

The CTES models present in this crate are generated from [CTES Protobuf](https://github.com/CiphertextEcosystem/CiphertextEcosystemProtobuf). The protobuf defines models that are generated across clients for many languages.

## Quickstart

You can build the generated structures using `make build`.

## Example usage

`cargo add ctes-models` will add this crate to your repo.

Examples:

```rs
use ctes_models::{
    Ciphertext,
    CiphertextMetadata,
    EncodingMetadata,
    Encoding
};

fn main() {
    // Creates encoding metadata
    let enc_metadata: EncodingMetadata = EncodingMetadata{
        encoding: Encoding::Utf8 as i32,
        base: 0,
    };
    
    // Creates ciphertext metadata
    let ct_metadata: CiphertextMetadata = CiphertextMetadata{
        encoding: Some(enc_metadata),
        // Note: "type" is a reserved word so you must use r#type
        r#type: "text".to_string()
    };

    // Creates a ciphertext
    let ciphertext: Ciphertext = Ciphertext{
        bytes: "Hello, world!".as_bytes().to_vec(),
        metadata: Some(ct_metadata)
    };

    println!("Ciphertext: {:?}", ciphertext);
}
```
