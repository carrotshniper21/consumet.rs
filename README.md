# Usage

## Movies

### Flixhq

To use the `FlixHQ` crate, you can follow this example:

```rust
use consumet_api_rs::models::movie_parser::MovieParser;
use consumet_api_rs::providers::movies;

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

# TODO

Please note that currently, only the searching functionality for the flixhq movie provider is available. Additional functionality, such as retrieving detailed movie information or accessing other providers, will be added in future updates. 
