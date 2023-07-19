# Usage

### Cargo.toml
```toml
reqwest = "0.11.18"
tokio = { version = "1.29.1", features = ["full"] }
```

## Movies

### Flixhq

To use the `FlixHQ` crate, you can follow this example:

```rust
use pipebomb_extensions_rs::models::movie_parser::MovieParser;
use pipebomb_extensions_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.search("hi".to_owned(), Some(1)).await?
    );

    Ok(())
}
```
The `movies::FlixHQ.search` function is used to search for movies with the example search query `"hi"` and an optional page number `1`. Then results are printed using the println! macro.

Please note that currently, only the searching functionality for the flixhq movie provider is available. Additional functionality, such as retrieving detailed movie information or accessing other movie providers, will be added in future updates. Stay tuned for more features and updates!
