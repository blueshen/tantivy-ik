<!-- Badges section here. -->
[![Crates.io](https://img.shields.io/badge/crates.io-0.6.2-green)](https://crates.io/crates/tantivy-ik)
[![License](https://img.shields.io/badge/license-LGPL--2.1-blue)](./LICENSE)
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/blueshen/tantivy-ik/releases)
[![Rust](https://github.com/blueshen/tantivy-ik/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/blueshen/tantivy-ik/actions/workflows/rust.yml)

[![GitHub forks](https://img.shields.io/github/forks/blueshen/tantivy-ik.svg?style=social&label=Fork)](https://github.com/blueshen/tantivy-ik/network/members)
[![GitHub stars](https://img.shields.io/github/stars/blueshen/tantivy-ik.svg?style=social&label=Star)](https://github.com/blueshen/tantivy-ik/stargazers)
<!-- /Badges section end. -->

# tantivy-ik

[ik-rs](https://github.com/blueshen/ik-rs) for [Tantivy](https://github.com/quickwit-oss/tantivy)

# Usage for Tantivy

Cargo.toml
```toml
[dependencies]
tantivy-ik = "0.6.2"
```

```rust

mod tests {
    use ik_rs::core::ik_segmenter::TokenMode;
    use ik_rs::IkTokenizer;
    use tantivy::Index;
    use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions};

    #[test]
    fn it_works() {
        let mut schema_builder = Schema::builder();
        let text_field_indexing = TextFieldIndexing::default()
            .set_tokenizer("ik-index")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_field_indexing)
            .set_stored();
        schema_builder.add_text_field("title", text_options);
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());
        index
            .tokenizers()
            .register("ik-index", IkTokenizer::new(TokenMode::INDEX));
        index
            .tokenizers()
            .register("ik-search", IkTokenizer::new(TokenMode::SEARCH));
    }
}
```
---
Welcome rust developer and search engine developer join us, and maintain this project together!

you can PR or submit issue...

and star⭐️ or fork this project to support me!